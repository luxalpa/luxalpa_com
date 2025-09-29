use crate::app::MyGlobalRes;
use crate::common::projects::ProjectMetadata;
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let projects_list = move || {
        let res = expect_context::<MyGlobalRes>().projects;

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
