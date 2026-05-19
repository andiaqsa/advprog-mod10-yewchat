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
                let json = serde_json::to_string(&register).unwrap();
                let _ = ws_open.send_with_str(&json);
            });
            ws_ref.set_onopen(Some(onopen.as_ref().unchecked_ref()));
            onopen.forget();

            // onmessage: simpan ke Rc<RefCell> lalu update state
            let msg_list_clone = msg_list.clone();
            let user_list_clone = user_list.clone();
            let messages_state = messages.clone();
            let users_state = users.clone();

            let onmessage = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
                if let Some(txt) = e.data().as_string() {
                    if let Ok(server_msg) = serde_json::from_str::<ServerMessage>(&txt) {
                        match server_msg.message_type.as_str() {
                            "users" => {
                                let mut list = user_list_clone.borrow_mut();
                                *list = server_msg.data_array.clone();
                                users_state.set(list.clone());
                            }
                            "message" => {
                                if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&server_msg.data) {
                                    let mut list = msg_list_clone.borrow_mut();
                                    list.push(chat_msg);
                                    messages_state.set(list.clone());
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
                let json = serde_json::to_string(&msg).unwrap();
                let _ = ws.send_with_str(&json);
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

    html! {
        <div class="flex h-screen bg-gray-900 text-white">
            <div class="w-48 bg-gray-800 border-r border-gray-700 flex flex-col">
                <div class="px-4 py-3 border-b border-gray-700">
                    <p class="text-sm font-bold text-gray-300">{"Online"}</p>
                </div>
                <div class="flex-1 overflow-y-auto p-2 space-y-1">
                    { for (*users).iter().map(|u| html! {
                        <div class="flex items-center gap-2 px-3 py-2 rounded-lg bg-gray-700">
                            <span class="w-2 h-2 bg-green-400 rounded-full"></span>
                            <span class="text-sm truncate">{u}</span>
                        </div>
                    })}
                </div>
            </div>

            <div class="flex flex-col flex-1">
                <div class="bg-gray-800 px-6 py-4 border-b border-gray-700 flex items-center gap-3">
                    <span class="text-xl">{"💬"}</span>
                    <div>
                        <h1 class="font-bold">{"YewChat"}</h1>
                        <p class="text-gray-400 text-xs">{"Logged in as: "}{&username}</p>
                    </div>
                </div>

                <div class="flex-1 overflow-y-auto p-4 space-y-3">
                    { for (*messages).iter().map(|msg| {
                        let is_me = msg.from == username;
                        html! {
                            <div class={if is_me { "flex justify-end" } else { "flex justify-start" }}>
                                <div class={if is_me {
                                    "bg-blue-600 px-4 py-2 rounded-2xl rounded-br-sm max-w-xs"
                                } else {
                                    "bg-gray-700 px-4 py-2 rounded-2xl rounded-bl-sm max-w-xs"
                                }}>
                                    if !is_me {
                                        <p class="text-xs text-gray-400 mb-1 font-semibold">{&msg.from}</p>
                                    }
                                    <p class="text-sm">{&msg.message}</p>
                                </div>
                            </div>
                        }
                    })}
                </div>

                <div class="bg-gray-800 border-t border-gray-700 px-4 py-3 flex gap-3">
                    <input
                        type="text"
                        placeholder="Ketik pesan..."
                        class="flex-1 bg-gray-700 text-white px-4 py-2 rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
                        value={(*input).clone()}
                        oninput={oninput}
                        onkeypress={onkeypress}
                    />
                    <button onclick={onsend}
                        class="bg-blue-600 hover:bg-blue-700 px-5 py-2 rounded-full font-semibold text-sm transition">
                        {"Kirim"}
                    </button>
                </div>
            </div>
        </div>
    }
}