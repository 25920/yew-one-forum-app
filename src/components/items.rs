use yew::prelude::*;

use crate::store::Feedback;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub feedback: Feedback,
}

#[function_component]
pub fn FeedbackItem(props: &Props) -> Html {
    html! {
        <div class="bg-white text-gray-700 rounded-lg p-8 my-5 relative">
            <div class="bg-pink-500 text-white rounded-full border-2 border-gray-200 w-12 h-12 flex items-center justify-center text-2xl font-bold absolute top-0 left-0 -mt-4 -ml-4">
                {props.feedback.rating}
            </div>
            <p>
                {&props.feedback.text}
            </p>
        </div>
    }
}

