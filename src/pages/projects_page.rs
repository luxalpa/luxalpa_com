use crate::common::projects::{fetch_projects, ProjectMetadata};
use leptos::prelude::*;
use leptos_fetch::QueryClient;
use leptos_meta::Title;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let client: QueryClient = expect_context();

    let res = client.resource_blocking(fetch_projects, || ());

    let projects_list = move || {
        // TODO: Error handling
        let projects = res.get()?.unwrap_or_default();

        Some(view! {
            <ul class="entity-list">
                {projects.into_iter().map(|project| view! {
                    <ProjectAbstract project=project.clone() />
                }).collect_view()}
            </ul>
        })
    };

    view! {
        <div class="projects-page page">
            <Title text="Projects"/>
            <h1>"Projects"</h1>
            <Suspense>
                {projects_list}
            </Suspense>
        </div>
    }
}

#[component]
fn ProjectAbstract(project: ProjectMetadata) -> impl IntoView {
    view! {
        <li>
            <a href={format!("/projects/{}", project.slug)}>
                <h2>{project.title}</h2>
                <div class="date">{project.date}</div>
                <div class="description">{project.description}</div>
            </a>
        </li>
    }
}
