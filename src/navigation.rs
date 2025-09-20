use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="navigation">
            <div class="logo-title">
                <img src="/assets/logo-small.jpg" alt="logo" class="logo-small" />
                <div class="logo-text">"Luxalpa's Lair"</div>
            </div>

            <div class="navlinks">
                <A href="/">"Home"</A>
                <A href="/blog">"Blog"</A>
                <A href="/projects">"Projects"</A>
                <A href="/resume">"Resume"</A>
                <A href="/about">"About"</A>
            </div>
        </nav>
    }
}
