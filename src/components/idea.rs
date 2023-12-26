use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Post;
use crate::store::Store;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub post: Post,
}

#[function_component]
pub fn PostItem(props: &Props) -> Html {
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
            <input type="text" class="bg-gray-50 mt-7 border border-gray-300 text-gray-900 text-sm 
            rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600
             dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 
             dark:focus:border-blue-500" placeholder="Your Comment" />
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