use leptos::prelude::*;

#[component]
pub fn RichText(text: String) -> impl IntoView {
    let mdhtml = markdown::to_html(&text);
    view! {
        <div inner_html=mdhtml />
    }
}
