use crate::common::articles::fetch_articles;
use crate::common::projects::fetch_projects;
use crate::navigation::Navigation;
use crate::pages::about_page::AboutPage;
use crate::pages::article_page::ArticlePage;
use crate::pages::blog_page::BlogPage;
use crate::pages::home_page::HomePage;
use crate::pages::project_page::ProjectPage;
use crate::pages::projects_page::ProjectsPage;
use crate::pages::resume_page::ResumePage;
use leptos::prelude::*;
use leptos_fetch::QueryClient;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, SsrMode, StaticSegment, WildcardSegment,
};

#[derive(Clone, Copy)]
pub struct MyGlobalRes {
    pub articles:
        OnceResource<Result<Vec<crate::common::articles::ArticleMetadata>, ServerFnError>>,
    pub projects:
        OnceResource<Result<Vec<crate::common::projects::ProjectMetadata>, ServerFnError>>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    QueryClient::new().provide();

    let articles = OnceResource::new_blocking(async move { fetch_articles().await });
    let projects = OnceResource::new_blocking(async move { fetch_projects().await });

    let inner = move || {
        articles.track();
        projects.track();
    };

    let global_res = MyGlobalRes { articles, projects };
    provide_context(global_res);

    // let inner = move || {
    //     let client: QueryClient = expect_context();
    //     Suspend::new(async move {
    //         client.prefetch_query(fetch_articles, ()).await;
    //     })
    // };

    view! {
        <Stylesheet id="leptos" href="/pkg/luxalpa_com.css"/>

        <Suspense>
            {inner}
        </Suspense>

        <Title text="Welcome to Leptos"/>

        <Router>
            <Navigation />
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=path!("/blog") view=BlogPage ssr=SsrMode::PartiallyBlocked />
                    <Route path=path!("/articles/:id") view=ArticlePage ssr=SsrMode::PartiallyBlocked />
                    <Route path=path!("/projects") view=ProjectsPage />
                    <Route path=path!("/projects/:id") view=ProjectPage />
                    <Route path=path!("/resume") view=ResumePage />
                    <Route path=path!("/about") view=AboutPage />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
