use crate::components::icon::{Icon, IconName};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

#[component]
pub fn Navigation() -> impl IntoView {
    let menu_open = RwSignal::new(false);

    let toggle_menu = move |_| {
        menu_open.update(|open| *open = !*open);
    };

    let click_menu = move |_| {
        menu_open.set(false);
    };

    view! {
        <nav class="navigation">
            <div class="backdrop" />
            <div class="nav-content" class:opened=menu_open>
                <NavTitle />

                <Icon name=IconName::Menu class:menu=true on:click=toggle_menu />

                <div class="navlinks">
                    <A href="/" on:click=click_menu>"Home"</A>
                    // <A href="/blog">"Blog"</A>
                    <A href="/projects" on:click=click_menu>"Projects"</A>
                    // <A href="/resume">"Resume"</A>
                    <A href="/contact" on:click=click_menu>"Contact"</A>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn NavTitle() -> impl IntoView {
    let location = use_location();

    let title = move || {
        let path = location.pathname.get();
        current_title(&path)
    };

    view! {
        <div class="logo-title">
            <img src="/assets/logo-small.jpg" alt="logo" class="logo-small" />
            <h1 class="logo-text">{title}</h1>
        </div>
    }
}

fn current_title(path: &str) -> Oco<'static, str> {
    if path.starts_with("/articles/") {
        return "Luxalpa’s Blog".into();
    } else if path.starts_with("/projects/") {
        return "Luxalpa’s Projects".into();
    }

    let s = match path {
        "/" => "Lair",
        "/blog" => "Blog",
        "/projects" => "Projects",
        "/resume" => "Resume",
        "/contact" => return "Contact Luxalpa".into(),
        _ => "Lair",
    };

    format!("Luxalpa’s {}", s).into()
}
