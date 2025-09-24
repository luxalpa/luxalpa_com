use crate::common::projects::fetch_project;
use crate::components::richtext::RichText;
use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, Params)]
struct ProjectPageParams {
    id: String,
}

#[component]
pub fn ProjectPage() -> impl IntoView {
    let params = use_params::<ProjectPageParams>();

    let contents = move || {
        params.get().ok().map(|params| {
            view! {
                <ProjectContents project_id=params.id />
            }
        })
    };

    view! {
        <div class="project-page page">
            { contents }
        </div>
    }
}

#[component]
fn ProjectContents(project_id: String) -> impl IntoView {
    // TODO: Error handling
    let res = OnceResource::new_blocking(async { fetch_project(project_id).await.unwrap() });

    let contents = move || {
        let Some(project) = res.get() else {
            return EitherOf3::A(view! {
                "Loading..."
            });
        };

        let Some(project) = project else {
            return EitherOf3::B(view! {
                "Project not found."
            });
        };

        EitherOf3::C(view! {
            <Title text=project.title.clone() />
            <div>
                <h1>{project.title}</h1>
                <div><RichText text=project.content /></div>
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
