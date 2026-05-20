use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;


#[derive(Deserialize, Debug)]
struct ClientMessage {
    #[serde(rename = "messageType")]
    message_type: String,
    #[serde(default)]
    data: String,
}

#[derive(Serialize, Clone)]
struct ServerMessage {
    #[serde(rename = "messageType")]
    message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
    #[serde(rename = "dataArray", skip_serializing_if = "Option::is_none")]
    data_array: Option<Vec<String>>,
}

#[derive(Serialize)]
struct ChatData {
    from: String,
    message: String,
    time: u64,
}


type Users = Arc<Mutex<HashMap<String, String>>>; // addr → nick


#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🦀 Rust WebSocket Server berjalan di ws://{}", addr);
    println!("   Format pesan: JSON (kompatibel dengan YewChat)");

    let (tx, _rx) = broadcast::channel::<String>(256);
    let users: Users = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        println!("✅ Koneksi baru dari: {}", addr);

        let tx = tx.clone();
        let rx = tx.subscribe();
        let users = users.clone();
        let addr_str = addr.to_string();

        tokio::spawn(handle_connection(stream, addr_str, tx, rx, users));
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    addr: String,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
    users: Users,
) {
    let ws = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("❌ Gagal handshake: {}", e);
            return;
        }
    };

    let (mut write, mut read) = ws.split();
    let mut nick: Option<String> = None;

    loop {
        tokio::select! {
            msg = read.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        println!("📨 [{}] {}", addr, text);
                        handle_client_message(
                            &text, &addr, &mut nick,
                            &tx, &users
                        ).await;
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        println!("🔌 Disconnect: {}", addr);
                        break;
                    }
                    _ => {}
                }
            }

            broadcast = rx.recv() => {
                if let Ok(msg) = broadcast {
                    if write.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            }
        }
    }

    // Cleanup saat disconnect
    if let Some(n) = &nick {
        let mut map = users.lock().await;
        map.remove(&addr);
        println!("👋 {} ({}) keluar", n, addr);

        // Broadcast daftar user terbaru
        let nicks: Vec<String> = map.values().cloned().collect();
        let msg = ServerMessage {
            message_type: "users".to_string(),
            data: None,
            data_array: Some(nicks),
        };
        let _ = tx.send(serde_json::to_string(&msg).unwrap());
    }
}


async fn handle_client_message(
    raw: &str,
    addr: &str,
    nick: &mut Option<String>,
    tx: &broadcast::Sender<String>,
    users: &Users,
) {

    let client_msg: ClientMessage = match serde_json::from_str(raw) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("⚠ JSON tidak valid dari {}: {}", addr, e);
            return;
        }
    };

    match client_msg.message_type.as_str() {
        "register" => {
            let name = client_msg.data.trim().to_string();
            if name.is_empty() { return; }

            *nick = Some(name.clone());
            let mut map = users.lock().await;
            map.insert(addr.to_string(), name.clone());
            println!("📋 Register: {} sebagai \"{}\"", addr, name);

            let nicks: Vec<String> = map.values().cloned().collect();
            let msg = ServerMessage {
                message_type: "users".to_string(),
                data: None,
                data_array: Some(nicks),
            };
            let _ = tx.send(serde_json::to_string(&msg).unwrap());
        }

        "message" => {
            let sender = match nick {
                Some(n) => n.clone(),
                None => {
                    eprintln!("⚠ Pesan dari {} yang belum register", addr);
                    return;
                }
            };

            let chat_data = ChatData {
                from: sender,
                message: client_msg.data,
                time: current_time_ms(),
            };
            let data_str = serde_json::to_string(&chat_data).unwrap();

            let msg = ServerMessage {
                message_type: "message".to_string(),
                data: Some(data_str),
                data_array: None,
            };
            let json = serde_json::to_string(&msg).unwrap();
            println!("📢 Broadcast: {}", json);
            let _ = tx.send(json);
        }

        other => {
            println!("❓ messageType tidak dikenal: {}", other);
        }
    }
}

fn current_time_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}