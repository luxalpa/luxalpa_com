use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn ResumePage() -> impl IntoView {
    view! {
        <Title text="Resume"/>
        <div class="page">
            <h1>"Resume"</h1>
        </div>
    }
}
