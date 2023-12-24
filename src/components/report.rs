use super::{
    feedback::FeedbackForm,
    list::FeedbackList,
    display::FeedbackStats,
};
use crate::store::Store;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn FeedBackIndex() -> Html {
    let (store, _) = use_store::<Store>();
    let loading = &store.loading;

    html! {
        <>
            <main class="md:container mt-24 px-5">
                <FeedbackForm />
                <FeedbackStats />
                <FeedbackList />
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

