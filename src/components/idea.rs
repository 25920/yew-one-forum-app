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

    let on_submit = {
        let cloned_dispatch = dispatch.clone();
        let text = text.clone();
        let message = message.clone();
        let text_input_ref = text_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            let dispatch = cloned_dispatch.clone();
            event.prevent_default();
            set_loading(true, dispatch.clone());

            if text.trim().len() < *&3 {
                message.set(Some("Text must be at least 3 characters".to_string()));
                set_loading(false, dispatch.clone());
                return;
            }

            let new_feedback = Post {
                id: Uuid::new_v4(),
                text: text.to_string(),
                resps: Vec::new(),
            };

            set_comment(new_feedback, dispatch.clone());
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
                {&props.post.text}
            </p>
            <small>{"by "}{&props.post.id}</small>
            <ul>
            {
                comment_list.resps.iter().map(|c| {
                    let val = c.comment.to_string();
                    html! {
                    <li>
                      <p>{val}</p>
                    </li>
                  }}).collect::<Html>()
            }
            </ul>
            <form onsubmit={on_submit}>
                <input type="text" class="bg-gray-50 mt-7 border border-gray-300 text-gray-900 text-sm 
                rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600
                dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 
                dark:focus:border-blue-500" placeholder="Your Comment" />
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