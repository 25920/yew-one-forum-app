use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <h1>
            {"Build a Feedback App with Yew.rs and Rust"}
        </h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

