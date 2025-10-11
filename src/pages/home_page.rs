use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Home"/>
        <div class="page home-page">
            <p class="core">"I am Luxalpa."</p>
            <p class="core">
                "I design, build, refactor, test, debug, reverse-engineer and fix Applications, Websites, Plugins, Games and Mods."
            </p>
        </div>
    }
}
