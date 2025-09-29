use crate::app::MyGlobalRes;
use crate::common::projects::{fetch_project, ProjectMetadata};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_fetch::QueryClient;
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
    let slug = project.slug.clone();

    let prefetch_project = move |_| {
        let client: QueryClient = expect_context();

        let slug = slug.clone();

        spawn_local(async move {
            client.prefetch_query(fetch_project, slug).await;
        });
    };

    view! {
        <li>
            <a href={format!("/projects/{}", project.slug)} on:mouseenter=prefetch_project>
                <h2>{project.title}</h2>
                <div class="date">{project.date}</div>
                <div class="description">{project.description}</div>
            </a>
        </li>
    }
}
