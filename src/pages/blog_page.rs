use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn BlogPage() -> impl IntoView {
    let res = OnceResource::new_blocking(async { fetch_articles().await.unwrap() });

    let article_list = move || {
        let articles = res.get()?;

        Some(view! {
            <ul>
                {articles.into_iter().map(|article| view! {
                    <ArticleAbstract article=article.clone() />
                }).collect_view()}
            </ul>
        })
    };

    view! {
        <div class="blog-page">
            <Title text="Blog Articles"/>
            <h1>"Blog Articles"</h1>
            <Suspense>
                {article_list}
            </Suspense>
        </div>
    }
}

#[component]
fn ArticleAbstract(article: Article) -> impl IntoView {
    view! {
        <li>
            <a href={format!("/articles/{}", article.slug)}>
                <h2>{article.title}</h2>
                <div class="date">{article.date}</div>
                <div class="description">{article.description}</div>
            </a>
        </li>
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Article {
    pub date: String,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub content: String,
}

#[server]
pub async fn fetch_articles() -> Result<Vec<Article>, ServerFnError> {
    use leptos_actix::extract;
    use std::ops::Deref;

    let articles = extract::<actix_web::web::Data<Vec<Article>>>()
        .await?
        .deref()
        .deref()
        .clone();

    Ok(articles)
}
