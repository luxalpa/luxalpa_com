use leptos::prelude::*;
use leptos::server;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Project {
    pub date: String,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub content: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ProjectMetadata {
    pub date: String,
    pub title: String,

    #[serde(rename = "abstract")]
    pub description: String,
    pub slug: String,
}

impl From<Project> for ProjectMetadata {
    fn from(project: Project) -> Self {
        Self {
            date: project.date,
            title: project.title,
            description: project.description,
            slug: project.slug,
        }
    }
}

#[server]
pub async fn fetch_projects() -> Result<Vec<ProjectMetadata>, ServerFnError> {
    use leptos_actix::extract;
    use std::ops::Deref;

    let projects = extract::<actix_web::web::Data<Vec<Project>>>()
        .await?
        .deref()
        .deref()
        .clone();

    let metadata = projects.into_iter().map(ProjectMetadata::from).collect();

    Ok(metadata)
}

#[server]
pub async fn fetch_project(id: String) -> Result<Option<Project>, ServerFnError> {
    use leptos_actix::extract;

    let projects = extract::<actix_web::web::Data<Vec<Project>>>().await?;

    let project = projects.iter().find(|project| project.slug == id).cloned();

    Ok(project)
}

#[cfg(feature = "ssr")]
pub fn load_projects() -> anyhow::Result<Vec<Project>> {
    use crate::server::markdown::{parse_frontmatter, Document};

    let dir = "content/projects";
    let files = std::fs::read_dir(dir)?;
    let projects = files
        .map(|file| {
            let content = std::fs::read_to_string(file?.path())?;
            let document: Document<ProjectMetadata> =
                parse_frontmatter::<ProjectMetadata>(&content)?;
            Ok(Project {
                date: document.metadata.date,
                title: document.metadata.title,
                description: document.metadata.description,
                slug: document.metadata.slug,
                content: document.content,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(projects)
}
