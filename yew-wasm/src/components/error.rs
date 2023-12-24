use yew::prelude::*;

#[function_component]
pub fn Error() -> Html {

    html! {
        <div class="flex-inline items-center mb-11">
            <h4 class="text-white">{"404"}</h4>
            <a href="/" class="text-white">{"Back to Home"}</a>
        </div>
    }
}

