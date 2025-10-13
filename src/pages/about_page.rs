use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="Contact"/>
        <div class="page about-page">
            <h2>"Website owner / Seiteninhaber"</h2>
            <div class="address">
                <div>"Name:"</div><div>"Lux Piek"</div>
                <div>"Adresse:"</div>
                <div>
                    <div>"Seesener Straße 37"</div>
                    <div>"10711 Berlin"</div>
                    <div>"Germany"</div>
                </div>
                <div>"Tel:"</div><div>"+49 (0) 1522 879"<span style="display: none">"(Remove me!)"</span>" 3429"</div>
                <div>"E-Mail:"</div><div>"contact.lxc (at) luxalpa"<span style="display: none">"(Remove me!)"</span>".com"</div>
            </div>
        </div>
    }
}
