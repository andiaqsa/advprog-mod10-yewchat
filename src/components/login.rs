use yew::prelude::*;
use yew_router::prelude::*;
use super::app::Route;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(String::new);
    let navigator = use_navigator().unwrap();

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
        Callback::from(move |_| {
            if !username.is_empty() {
                // Simpan username ke sessionStorage
                let window = web_sys::window().unwrap();
                let storage = window.session_storage().unwrap().unwrap();
                storage.set_item("username", &username).unwrap();
                navigator.push(&Route::Chat);
            }
        })
    };

    html! {
        <div class="flex items-center justify-center min-h-screen bg-gray-900">
            <div class="bg-gray-800 p-8 rounded-2xl shadow-xl w-96">
                <h1 class="text-3xl font-bold text-white mb-2 text-center">
                    {"💬 YewChat"}
                </h1>
                <p class="text-gray-400 text-center mb-6">
                    {"Masukkan username untuk mulai chat"}
                </p>
                <input
                    type="text"
                    placeholder="Username kamu..."
                    class="w-full px-4 py-3 rounded-lg bg-gray-700 text-white border border-gray-600 focus:outline-none focus:border-blue-500 mb-4"
                    oninput={oninput}
                    value={(*username).clone()}
                />
                <button
                    onclick={onclick}
                    class="w-full py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg transition"
                >
                    {"Masuk ke Chat →"}
                </button>
            </div>
        </div>
    }
}