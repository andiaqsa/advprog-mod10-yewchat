use yew::prelude::*;
use web_sys::{WebSocket, MessageEvent, HtmlInputElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use super::types::{ClientMessage, ServerMessage, ChatMessage};
use std::rc::Rc;
use std::cell::RefCell;

#[function_component(Chat)]
pub fn chat() -> Html {
    let messages: UseStateHandle<Vec<ChatMessage>> = use_state(Vec::new);
    let users: UseStateHandle<Vec<String>> = use_state(Vec::new);
    let input = use_state(String::new);
    let show_about = use_state(|| false);

    let username = {
        let window = web_sys::window().unwrap();
        let storage = window.session_storage().unwrap().unwrap();
        storage.get_item("username").unwrap().unwrap_or_else(|| "Anonymous".to_string())
    };

    let ws = use_memo((), |_| {
        WebSocket::new("ws://localhost:8080").expect("Failed to connect")
    });

    {
        let messages = messages.clone();
        let users = users.clone();
        let ws_ref = ws.clone();
        let username_clone = username.clone();

        use_effect_with((), move |_| {
            let msg_list: Rc<RefCell<Vec<ChatMessage>>> = Rc::new(RefCell::new(Vec::new()));
            let user_list: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

            // onopen: kirim register
            let ws_open = ws_ref.clone();
            let uname = username_clone.clone();
            let onopen = Closure::<dyn FnMut()>::new(move || {
                let register = ClientMessage {
                    message_type: "register".to_string(),
                    data: uname.clone(),
                };
                let _ = ws_open.send_with_str(&serde_json::to_string(&register).unwrap());
            });
            ws_ref.set_onopen(Some(onopen.as_ref().unchecked_ref()));
            onopen.forget();

            // onmessage
            let msg_list_c = msg_list.clone();
            let user_list_c = user_list.clone();
            let msg_state = messages.clone();
            let usr_state = users.clone();

            let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
                if let Some(txt) = e.data().as_string() {
                    if let Ok(srv) = serde_json::from_str::<ServerMessage>(&txt) {
                        match srv.message_type.as_str() {
                            "users" => {
                                let mut list = user_list_c.borrow_mut();
                                *list = srv.data_array.clone();
                                usr_state.set(list.clone());
                            }
                            "message" => {
                                if let Ok(msg) = serde_json::from_str::<ChatMessage>(&srv.data) {
                                    let mut list = msg_list_c.borrow_mut();
                                    list.push(msg);
                                    msg_state.set(list.clone());
                                }
                            }
                            _ => {}
                        }
                    }
                }
            });
            ws_ref.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            onmessage.forget();

            let ws_close = ws_ref.clone();
            move || { let _ = ws_close.close(); }
        });
    }

    let oninput = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let el = e.target_unchecked_into::<HtmlInputElement>();
            input.set(el.value());
        })
    };

    let send_message = {
        let input = input.clone();
        let ws = ws.clone();
        Callback::from(move |_: ()| {
            let text = (*input).trim().to_string();
            if !text.is_empty() {
                let msg = ClientMessage {
                    message_type: "message".to_string(),
                    data: text,
                };
                let _ = ws.send_with_str(&serde_json::to_string(&msg).unwrap());
                input.set(String::new());
            }
        })
    };

    let onkeypress = {
        let send = send_message.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" { send.emit(()); }
        })
    };

    let onsend = {
        let send = send_message.clone();
        Callback::from(move |_: MouseEvent| { send.emit(()); })
    };

    let onclear = {
        let messages = messages.clone();
        Callback::from(move |_: MouseEvent| {
            messages.set(Vec::new());
        })
    };

    let toggle_about = {
        let show_about = show_about.clone();
        Callback::from(move |_: MouseEvent| {
            show_about.set(!*show_about);
        })
    };

    html! {
        <div class="flex h-screen bg-gray-900 text-white overflow-hidden">

            // Sidebar
            <div class="w-52 bg-gray-800 border-r border-gray-700 flex flex-col flex-shrink-0">
                // Logo di sidebar
                <div class="px-4 py-4 border-b border-gray-700">
                    <div class="flex items-center gap-2">
                        <span class="text-xl">{"💬"}</span>
                        <span class="font-bold text-white">{"YewChat"}</span>
                    </div>
                    <p class="text-xs text-gray-500 mt-1">{"🦀 Powered by Rust"}</p>
                </div>

                // Daftar user online
                <div class="px-3 py-3 flex-1 overflow-y-auto">
                    <p class="text-xs font-bold text-gray-400 uppercase tracking-wider mb-2 px-1">
                        {format!("Online — {}", (*users).len())}
                    </p>
                    { for (*users).iter().map(|u| {
                        let is_me = *u == username;
                        html! {
                            <div class="flex items-center gap-2 px-2 py-2 rounded-lg mb-1 hover:bg-gray-700 transition">
                                <div class="w-7 h-7 rounded-full bg-blue-600 flex items-center justify-center text-xs font-bold flex-shrink-0">
                                    { u.chars().next().unwrap_or('?').to_uppercase().to_string() }
                                </div>
                                <div class="flex-1 min-w-0">
                                    <p class="text-sm truncate">{u}</p>
                                    if is_me {
                                        <p class="text-xs text-blue-400">{"(kamu)"}</p>
                                    }
                                </div>
                                <span class="w-2 h-2 bg-green-400 rounded-full flex-shrink-0"></span>
                            </div>
                        }
                    })}
                </div>

                // Tombol di bawah sidebar
                <div class="p-3 border-t border-gray-700 space-y-2">
                    <button onclick={onclear}
                        class="w-full text-xs py-2 px-3 rounded-lg bg-gray-700 hover:bg-red-900 text-gray-300 hover:text-red-300 transition text-left flex items-center gap-2">
                        <span>{"🗑"}</span>{"Bersihkan Chat"}
                    </button>
                    <button onclick={toggle_about}
                        class="w-full text-xs py-2 px-3 rounded-lg bg-gray-700 hover:bg-blue-900 text-gray-300 hover:text-blue-300 transition text-left flex items-center gap-2">
                        <span>{"ℹ"}</span>{"Tentang Aplikasi"}
                    </button>
                </div>
            </div>

            // Area utama
            <div class="flex flex-col flex-1 min-w-0">

                // Header
                <div class="bg-gray-800 px-5 py-3 border-b border-gray-700 flex items-center justify-between flex-shrink-0">
                    <div class="flex items-center gap-3">
                        <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
                        <div>
                            <h1 class="font-bold text-sm">{"Ruang Obrolan Umum"}</h1>
                            <p class="text-gray-400 text-xs">{format!("{} pengguna aktif", (*users).len())}</p>
                        </div>
                    </div>
                    <div class="text-gray-500 text-xs bg-gray-700 px-3 py-1 rounded-full">
                        {"👤 "}{&username}
                    </div>
                </div>

                // Panel About
                if *show_about {
                    <div class="bg-blue-950 border-b border-blue-800 px-5 py-4 text-sm">
                        <h3 class="font-bold text-blue-300 mb-2">{"ℹ Tentang YewChat"}</h3>
                        <p class="text-gray-300 text-xs leading-relaxed">
                            {"YewChat adalah aplikasi chat real-time yang dibangun menggunakan "}
                            <span class="text-orange-400 font-semibold">{"Rust"}</span>
                            {" dan dikompilasi ke "}
                            <span class="text-yellow-400 font-semibold">{"WebAssembly (WASM)"}</span>
                            {" menggunakan framework "}
                            <span class="text-blue-400 font-semibold">{"Yew"}</span>
                            {". Komunikasi real-time menggunakan protokol "}
                            <span class="text-green-400 font-semibold">{"WebSocket"}</span>
                            {". Dibuat sebagai bagian dari Tutorial 3 — Pemrograman Lanjut 2025/2026."}
                        </p>
                    </div>
                }

                // Area pesan
                <div class="flex-1 overflow-y-auto p-4 space-y-2">
                    if (*messages).is_empty() {
                        <div class="flex flex-col items-center justify-center h-full text-center">
                            <div class="text-5xl mb-4">{"💬"}</div>
                            <p class="text-gray-500 font-medium">{"Belum ada pesan"}</p>
                            <p class="text-gray-600 text-sm mt-1">{"Jadilah yang pertama menyapa!"}</p>
                        </div>
                    }
                    { for (*messages).iter().map(|msg| {
                        let is_me = msg.from == username;
                        let time_str = msg.formatted_time();
                        html! {
                            <div class={if is_me { "flex justify-end items-end gap-2" } else { "flex justify-start items-end gap-2" }}>
                                // Avatar (hanya untuk pesan orang lain)
                                if !is_me {
                                    <div class="w-7 h-7 rounded-full bg-blue-700 flex items-center justify-center text-xs font-bold flex-shrink-0 mb-1">
                                        { msg.from.chars().next().unwrap_or('?').to_uppercase().to_string() }
                                    </div>
                                }
                                <div class="max-w-xs lg:max-w-sm">
                                    // Nama pengirim
                                    if !is_me {
                                        <p class="text-xs text-gray-400 mb-1 ml-1 font-semibold">{&msg.from}</p>
                                    }
                                    <div class={if is_me {
                                        "bg-blue-600 text-white px-4 py-2 rounded-2xl rounded-br-sm"
                                    } else {
                                        "bg-gray-700 text-white px-4 py-2 rounded-2xl rounded-bl-sm"
                                    }}>
                                        <p class="text-sm leading-relaxed">{&msg.message}</p>
                                    </div>
                                    // Timestamp
                                    <p class={if is_me {
                                        "text-xs text-gray-500 mt-1 text-right mr-1"
                                    } else {
                                        "text-xs text-gray-500 mt-1 ml-1"
                                    }}>
                                        {time_str}
                                    </p>
                                </div>
                            </div>
                        }
                    })}
                </div>

                // Input area
                <div class="bg-gray-800 border-t border-gray-700 px-4 py-3 flex-shrink-0">
                    <div class="flex gap-3 items-center">
                        <input
                            type="text"
                            placeholder="Ketik pesan... (Enter untuk kirim)"
                            class="flex-1 bg-gray-700 text-white px-4 py-2 rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm placeholder-gray-500 border border-gray-600 focus:border-blue-500"
                            value={(*input).clone()}
                            oninput={oninput}
                            onkeypress={onkeypress}
                            maxlength="500"
                        />
                        <button onclick={onsend}
                            class="bg-blue-600 hover:bg-blue-500 active:scale-95 px-5 py-2 rounded-full font-semibold text-sm transition-all flex-shrink-0 shadow-lg">
                            {"Kirim ➤"}
                        </button>
                    </div>
                    <p class="text-xs text-gray-600 mt-2 text-center">
                        {"YewChat • Rust + WebAssembly + WebSocket"}
                    </p>
                </div>
            </div>
        </div>
    }
}