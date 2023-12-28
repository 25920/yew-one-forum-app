use yew::prelude::*;
use yewdux::prelude::*;
use crate::store::Post;
use crate::store::Store;
use crate::store::Comment;
use crate::store::set_comment;
use crate::store::set_loading;
use crate::store::set_show_alert;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub post: Post,
}

#[derive(Debug, PartialEq, Properties)]
pub struct CProps {
    pub comment: Comment,
}

#[function_component]
pub fn PostItem(props: &Props) -> Html {
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

    // let post_group = &props.post;

    let on_submit={
        let cloned_dispatch = dispatch.clone();
        let text = text.clone();
        let message = message.clone();
        let text_input_ref = text_input_ref.clone();
        let point = props.post.clone();

        Callback::from(move |event: SubmitEvent| {
            let dispatch = cloned_dispatch.clone();
            let post_group = point.clone();
            event.prevent_default();
            set_loading(true, dispatch.clone());

            if text.trim().len() < *&3 {
                message.set(Some("Text must be at least 3 characters".to_string()));
                set_loading(false, dispatch.clone());
                return;
            }

            let new_comment = Comment {
                id: Uuid::new_v4(),
                comment: text.to_string(),
            };

            set_comment(new_comment, dispatch.clone(),post_group);
            set_show_alert("Post added successfully".to_string(

            ), dispatch.clone());

            let text_input = text_input_ref.cast::<HtmlInputElement>().unwrap();
            text_input.set_value("");
            text.set(String::new());
            set_loading(false, dispatch);
        })
    };

    let comment_list = &props.post.clone();
    html! {
        <div class="bg-white text-gray-700 rounded-lg p-8 my-5 relative">
            <p>
                <strong>{&props.post.text}</strong>
            </p>
            <em>{"Replies:"}</em>
            <ul>
            {
                comment_list.resps.iter().map(|c| {
                    let val = c.comment.to_string();
                    html! {
                    <li>
                      <p>{val}</p>
                      <span class="flex border rounded-md h-3 bg-black-400"></span>
                    </li>
                  }}).collect::<Html>()
            }
            </ul>
            <form onsubmit={on_submit}>
                <div  class="flex border rounded-lg my-4 px-2 py-3">
                    <input type="text" 
                        ref={text_input_ref}
                        oninput={handle_input}
                        class="flex-grow border-none text-lg focus:outline-none" 
                        placeholder="Your Comment" 
                    />
                    <button
                        type="submit"
                        class={format!(
                            "border-0 rounded-md w-28 h-10 cursor-pointer 
                            hover:bg-red-500 {}",
                            if *loading { "bg-[#ccc] text-black-800"} else 
                            {"bg-teal-600 text-white"}
                        )}
                    >
                        {"Comment"}
                    </button>
                </div>
            </form>
        </div>
    }
}


#[function_component]
pub fn PostList() -> Html {
    let (store, _) = use_store::<Store>();
    let post_list = store.posts.clone();

    html! {
        <div>
            {
                post_list.into_iter().map(|post|{
                    let key = post.id.to_string();
                    html!{<PostItem {key} post={post.clone()} />}
                }).collect::<Html>()
            }
        </div>
    }
}