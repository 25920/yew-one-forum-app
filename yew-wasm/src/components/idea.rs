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
                {&props.post.text} {&props.post.id}
            </p>
            <ul>
            {
                comment_list.resps.iter().map(|c| html! {
                    <li>
                      <p>{c.comment}</p>
                    </li>
                  }).collect::<Html>()
            }
            </ul>
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