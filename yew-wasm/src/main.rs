mod components;
mod store;

use components::{
    post::PostForm,
    idea::PostList,
    report::FeedBackIndex,
    error::Error,
    nav::TopNav,
};
use store::Store;
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/feedback")]
    Report,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Report => html! {
            <FeedBackIndex />
        },
        Route::NotFound => html! { <Error/> },
    }
}

#[function_component]
fn Home() -> Html {
    let (store, _) = use_store::<Store>();
    let loading = &store.loading;

    html! {
        <>
            <main class="md:container mt-24 px-5">
                <PostForm />
                <PostList />
            </main>
            if *loading {
                <div
                    class="fixed top-5 left-5 inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-yellow-400 border-r-transparent align-[-0.125em] text-warning motion-reduce:animate-[spin_1.5s_linear_infinite]"
                    role="status">
                    <span
                    class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                    >{"Loading..."}</span
                >
                </div>
            }
        </>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <TopNav/>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

