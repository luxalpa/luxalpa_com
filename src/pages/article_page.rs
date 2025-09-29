use crate::common::articles::fetch_article;
use crate::components::richtext::RichText;
use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_fetch::QueryClient;
use leptos_meta::Title;
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
        <div class="article-page page">
            { contents }
        </div>
    }
}

#[component]
fn ArticleContents(article_id: String) -> impl IntoView {
    // TODO: Error handling
    let client: QueryClient = expect_context();

    let res = client.resource_blocking(fetch_article, move || article_id.clone());

    let contents = move || {
        let Some(article) = res.get() else {
            return EitherOf3::A(view! {
                "Loading..."
            });
        };

        let Ok(Some(article)) = article else {
            return EitherOf3::B(view! {
                "Article not found."
            });
        };

        EitherOf3::C(view! {
            <Title text=article.title.clone() />
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
