use crate::common::articles::{fetch_articles, ArticleMetadata};
use leptos::prelude::*;
use leptos::reactive::spawn_local_scoped;
use leptos_fetch::QueryClient;
use leptos_meta::Title;

#[component]
pub fn BlogPage() -> impl IntoView {
    let client = expect_context::<QueryClient>();
    let res = client.resource_blocking(fetch_articles, || ());

    let article_list = move || {
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
    let slug = article_meta.slug.clone();

    let prefetch_article = move |_| {
        let client: QueryClient = expect_context();

        let slug = slug.clone();

        spawn_local_scoped(async move {
            client
                .prefetch_query(crate::common::articles::fetch_article, slug)
                .await;
        });
    };

    view! {
        <li>
            <a href={format!("/articles/{}", article_meta.slug)} on:mouseenter=prefetch_article>
                <h2>{article_meta.title}</h2>
                <div class="date">{article_meta.date}</div>
                <div class="description">{article_meta.description}</div>
            </a>
        </li>
    }
}
