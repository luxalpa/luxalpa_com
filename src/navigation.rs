use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

#[component]
pub fn Navigation() -> impl IntoView {
    let location = use_location();

    let title = move || {
        let path = location.pathname.get();
        if path.starts_with("/articles/") {
            return "Luxalpa’s Blog".to_string();
        } else if path.starts_with("/projects/") {
            return "Luxalpa’s Projects".to_string();
        }

        let s = match path.as_str() {
            "/" => "Lair",
            "/blog" => "Blog",
            "/projects" => "Projects",
            "/resume" => "Resume",
            "/contact" => return "Contact Luxalpa".to_string(),
            _ => "Lair",
        };

        format!("Luxalpa’s {}", s)
    };

    view! {
        <nav class="navigation">
            <div class="logo-title">
                <img src="/assets/logo-small.jpg" alt="logo" class="logo-small" />
                <h1 class="logo-text">{title}</h1>
            </div>

            <div class="navlinks">
                <A href="/">"Home"</A>
                <A href="/blog">"Blog"</A>
                <A href="/projects">"Projects"</A>
                // <A href="/resume">"Resume"</A>
                <A href="/contact">"Contact"</A>
            </div>
        </nav>
    }
}
