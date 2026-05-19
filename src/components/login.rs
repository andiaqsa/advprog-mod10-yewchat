use yew::prelude::*;
use yew_router::prelude::*;
use super::app::Route;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(String::new);
    let navigator = use_navigator().unwrap();
    let error = use_state(String::new);

    let oninput = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let name = (*username).trim().to_string();
            if name.is_empty() {
                error.set("Username tidak boleh kosong!".to_string());
                return;
            }
            if name.len() < 3 {
                error.set("Username minimal 3 karakter!".to_string());
                return;
            }
            let window = web_sys::window().unwrap();
            let storage = window.session_storage().unwrap().unwrap();
            storage.set_item("username", &name).unwrap();
            navigator.push(&Route::Chat);
        })
    };

    let onkeypress = {
        let username = username.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let name = (*username).trim().to_string();
                if name.len() >= 3 {
                    let window = web_sys::window().unwrap();
                    let storage = window.session_storage().unwrap().unwrap();
                    storage.set_item("username", &name).unwrap();
                    navigator.push(&Route::Chat);
                }
            }
        })
    };

    html! {
        <div class="min-h-screen bg-gradient-to-br from-gray-900 via-blue-950 to-gray-900 flex items-center justify-center px-4">
            <div class="w-full max-w-md">

                // Logo & Judul
                <div class="text-center mb-8">
                    <div class="text-6xl mb-4">{"💬"}</div>
                    <h1 class="text-4xl font-bold text-white mb-2">{"YewChat"}</h1>
                    <p class="text-blue-300 text-sm">{"Dibangun dengan Rust + WebAssembly"}</p>
                </div>

                // Card login
                <div class="bg-gray-800 bg-opacity-80 rounded-2xl shadow-2xl p-8 border border-gray-700">

                    <h2 class="text-white font-semibold text-lg mb-1">{"Selamat Datang! 👋"}</h2>
                    <p class="text-gray-400 text-sm mb-6">
                        {"Masukkan username untuk bergabung ke ruang obrolan"}
                    </p>

                    <div class="mb-4">
                        <label class="block text-gray-400 text-xs font-semibold mb-2 uppercase tracking-wider">
                            {"Username"}
                        </label>
                        <input
                            type="text"
                            placeholder="Contoh: budi123"
                            class="w-full px-4 py-3 rounded-xl bg-gray-700 text-white border border-gray-600 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 transition placeholder-gray-500"
                            oninput={oninput}
                            onkeypress={onkeypress}
                            value={(*username).clone()}
                            maxlength="20"
                        />
                        if !(*error).is_empty() {
                            <p class="text-red-400 text-xs mt-2">{"⚠ "}{&*error}</p>
                        }
                    </div>

                    <button
                        onclick={onclick}
                        class="w-full py-3 bg-blue-600 hover:bg-blue-500 active:bg-blue-700 text-white font-bold rounded-xl transition-all duration-200 shadow-lg hover:shadow-blue-500/25"
                    >
                        {"Masuk ke Chat →"}
                    </button>

                    // Divider
                    <div class="flex items-center my-5">
                        <div class="flex-1 h-px bg-gray-700"></div>
                        <span class="px-3 text-gray-500 text-xs">{"INFO"}</span>
                        <div class="flex-1 h-px bg-gray-700"></div>
                    </div>

                    // Info badges
                    <div class="grid grid-cols-3 gap-3 text-center">
                        <div class="bg-gray-700 rounded-xl p-3">
                            <div class="text-2xl mb-1">{"⚡"}</div>
                            <p class="text-gray-400 text-xs">{"Real-time"}</p>
                        </div>
                        <div class="bg-gray-700 rounded-xl p-3">
                            <div class="text-2xl mb-1">{"🦀"}</div>
                            <p class="text-gray-400 text-xs">{"Rust/WASM"}</p>
                        </div>
                        <div class="bg-gray-700 rounded-xl p-3">
                            <div class="text-2xl mb-1">{"🔒"}</div>
                            <p class="text-gray-400 text-xs">{"WebSocket"}</p>
                        </div>
                    </div>
                </div>

                // Quote motivasi
                <p class="text-center text-gray-600 text-xs mt-6">
                    {"\"Kreativitas adalah kecerdasan yang sedang bersenang-senang.\" — Albert Einstein"}
                </p>
            </div>
        </div>
    }
}