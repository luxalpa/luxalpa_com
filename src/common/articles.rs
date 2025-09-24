use leptos::prelude::*;
use leptos::server;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Article {
    pub date: String,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub content: String,
}

#[derive(serde::Deserialize)]
#[cfg(feature = "ssr")]
pub struct ArticleMetadata {
    pub date: String,
    pub title: String,

    #[serde(rename = "abstract")]
    pub description: String,
    pub slug: String,
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

#[server]
pub async fn fetch_article(id: String) -> Result<Option<Article>, ServerFnError> {
    use leptos_actix::extract;

    let articles = extract::<actix_web::web::Data<Vec<Article>>>().await?;

    let article = articles.iter().find(|article| article.slug == id).cloned();

    Ok(article)
}

#[cfg(feature = "ssr")]
pub fn load_articles() -> anyhow::Result<Vec<Article>> {
    use crate::server::markdown::{parse_frontmatter, Document};

    let dir = "content/articles";
    let files = std::fs::read_dir(dir)?;
    let articles = files
        .map(|file| {
            let content = std::fs::read_to_string(file?.path())?;
            let document: Document<ArticleMetadata> =
                parse_frontmatter::<ArticleMetadata>(&content)?;
            Ok(Article {
                date: document.metadata.date,
                title: document.metadata.title,
                description: document.metadata.description,
                slug: document.metadata.slug,
                content: document.content,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(articles)
}
