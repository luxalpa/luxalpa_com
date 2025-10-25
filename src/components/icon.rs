use leptos::prelude::*;

#[allow(dead_code)] // TODO: Remove all that we don't use!
pub enum IconName {
    Menu,
    Close,
}

impl IconName {
    fn svg(&self) -> &'static str {
        match self {
            IconName::Menu => include_str!("../../assets/icons/menu.svg"),
            IconName::Close => include_str!("../../assets/icons/close.svg"),
        }
    }
}

#[component]
pub fn Icon(name: IconName) -> impl IntoView {
    let svg = name.svg();

    view! {
        <div class="icon" inner_html=svg></div>
    }
}
