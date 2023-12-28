use crate::store::{set_post, set_loading, set_show_alert, Post, Store};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn PostForm() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let loading = &store.loading;
    let text = use_state(String::new);
    let message = use_state(|| Option::<String>::None);

    let text_input_ref = use_node_ref();

    let handle_input = {
        let text = text.clone();
        let message = message.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            message.set(None);
            text.set(value);
        })
    };

    let on_submit = {
        let cloned_dispatch = dispatch.clone();
        let text = text.clone();
        let message = message.clone();
        let text_input_ref = text_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            let dispatch = cloned_dispatch.clone();
            event.prevent_default();
            set_loading(true, dispatch.clone());

            if text.trim().len() < *&13 {
                message.set(Some("Text must be at least 13 characters".to_string()));
                set_loading(false, dispatch.clone());
                return;
            }

            let new_post = Post {
                id: Uuid::new_v4(),
                text: text.to_string(),
                resps: Vec::new(),
            };

            set_post(new_post, dispatch.clone());
            set_show_alert("Post added successfully".to_string(

            ), dispatch.clone());

            let text_input = text_input_ref.cast::<HtmlInputElement>().unwrap();
            text_input.set_value("");
            text.set(String::new());
            set_loading(false, dispatch);
        })
    };

    html! {
        <div class="bg-white text-gray-700 rounded-lg p-8 my-5 relative">
            <header class="max-w-md mx-auto">
                <h2 class="text-center text-2xl font-bold">
                {"Anything you want to share?"}</h2>
            </header>
            <form onsubmit={on_submit}>
                <div class="flex border rounded-lg my-4 px-2 py-3">
                    <input
                        type="text"
                        ref={text_input_ref}
                        oninput={handle_input}
                        class="flex-grow border-none text-lg focus:outline-none"
                        placeholder="Tell us something"
                    />
                <button
                    type="submit"
                    class={format!(
                        "border-0 rounded-md w-28 h-10 cursor-pointer 
                        hover:bg-indigo-500 {}",
                        if *loading { "bg-[#ccc] text-gray-800"} else 
                        {"bg-indigo-600 text-white"}
                    )}
                >
                    {"Send"}
                </button>
                </div>
                {if let Some(msg) = message.as_ref() {
                    html! { <div class="pt-3 text-center
                     text-purple-600">{"mps: "} {msg.clone()}</div> }
                } else {
                    html! {}
                }}
            </form>
        </div>
    }
}

