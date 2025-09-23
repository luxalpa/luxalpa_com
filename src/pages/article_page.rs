use crate::components::richtext::RichText;
use crate::pages::blog_page::Article;
use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, Params)]
struct ArticlePageParams {
    id: String,
}

#[component]
pub fn ArticlePage() -> impl IntoView {
    let params = use_params::<ArticlePageParams>();

    let contents = move || {
        params.get().ok().map(|params| {
            view! {
                <ArticleContents article_id=params.id />
            }
        })
    };

    view! {
        <div class="article-page">
            { contents }
        </div>
    }
}

#[component]
fn ArticleContents(article_id: String) -> impl IntoView {
    // TODO: Error handling
    let res = OnceResource::new_blocking(async { fetch_article(article_id).await.unwrap() });

    let contents = move || {
        let Some(article) = res.get() else {
            return EitherOf3::A(view! {
                "Loading..."
            });
        };

        let Some(article) = article else {
            return EitherOf3::B(view! {
                "Article not found."
            });
        };

        EitherOf3::C(view! {
            <div>
                <h1>{article.title}</h1>
                <div><RichText text=article.content /></div>
            </div>
        })
    };

    view! {
        <div>
            <Suspense>
                { contents }
            </Suspense>
        </div>
    }
}

#[server]
pub async fn fetch_article(id: String) -> Result<Option<Article>, ServerFnError> {
    use leptos_actix::extract;

    let articles = extract::<actix_web::web::Data<Vec<Article>>>().await?;

    let article = articles.iter().find(|article| article.slug == id).cloned();

    Ok(article)
}
