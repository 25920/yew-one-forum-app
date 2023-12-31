use super::rating::Rating;
use crate::store::{set_feedback, set_loading, set_show_alert, Feedback, Store};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn FeedbackForm() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let loading = &store.loading;
    let text = use_state(String::new);
    let rating = use_state(|| 10_u8);
    let min = use_state(|| 10);
    let message = use_state(|| Option::<String>::None);

    let text_input_ref = use_node_ref();

    let handle_select = {
        let rating = rating.clone();
        Callback::from(move |value| {
            rating.set(value);
        })
    };

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
        let rating = rating.clone();
        let text = text.clone();
        let message = message.clone();
        let text_input_ref = text_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            let dispatch = cloned_dispatch.clone();
            event.prevent_default();
            set_loading(true, dispatch.clone());

            if text.trim().len() < *min {
                message.set(Some("Text must be at least 10 characters".to_string()));
                set_loading(false, dispatch.clone());
                return;
            }

            let new_feedback = Feedback {
                id: Uuid::new_v4(),
                text: text.to_string(),
                rating: *rating,
            };

            set_feedback(new_feedback, dispatch.clone());
            set_show_alert("Feeback added successfully".to_string(), dispatch.clone());

            let text_input = text_input_ref.cast::<HtmlInputElement>().unwrap();
            text_input.set_value("");
            text.set(String::new());
            rating.set(10);
            set_loading(false, dispatch);
        })
    };

    html! {
        <div class="bg-white text-gray-700 rounded-lg p-8 my-5 relative">
            <header class="max-w-md mx-auto">
                <h2 class="text-center text-2xl font-bold">{"How would you rate this service?"}</h2>
            </header>
            <form onsubmit={on_submit}>
                <Rating selected={*rating} onchange={handle_select} />
                <div class="flex border rounded-lg my-4 px-2 py-3">
                    <input
                        type="text"
                        ref={text_input_ref}
                        oninput={handle_input}
                        class="flex-grow border-none text-lg focus:outline-none"
                        placeholder="Tell us something that keeps you coming back"
                    />
                <button
                    type="submit"
                    class={format!(
                        "border-0 rounded-md w-28 h-10 cursor-pointer hover:bg-indigo-500 {}",
                        if *loading { "bg-[#ccc] text-gray-800"} else {"bg-indigo-600 text-white"}
                    )}
                >
                    {"Send"}
                </button>
                </div>
                {if let Some(msg) = message.as_ref() {
                    html! { <div class="pt-3 text-center text-purple-600">{msg.clone()}</div> }
                } else {
                    html! {}
                }}
            </form>
        </div>
    }
}

