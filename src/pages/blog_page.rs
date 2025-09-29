use crate::app::MyGlobalRes;
use crate::common::articles::ArticleMetadata;
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn BlogPage() -> impl IntoView {
    let article_list = move || {
        let res = expect_context::<MyGlobalRes>().articles;

        let articles = res.get()?.unwrap_or_default();

        Some(view! {
            <ul class="entity-list">
                {articles.into_iter().map(|article_meta| view! {
                    <ArticleAbstract article_meta=article_meta.clone() />
                }).collect_view()}
            </ul>
        })
    };

    view! {
        <div class="blog-page page">
            <Title text="Blog Articles"/>
            <h1>"Blog Articles"</h1>
            <Suspense>
                {article_list}
            </Suspense>
        </div>
    }
}

#[component]
fn ArticleAbstract(article_meta: ArticleMetadata) -> impl IntoView {
    view! {
        <li>
            <a href={format!("/articles/{}", article_meta.slug)}>
                <h2>{article_meta.title}</h2>
                <div class="date">{article_meta.date}</div>
                <div class="description">{article_meta.description}</div>
            </a>
        </li>
    }
}
