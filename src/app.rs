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

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let client = QueryClient::new().provide();

    client.resource_blocking(fetch_articles, || ());
    client.resource_blocking(fetch_projects, || ());

    view! {
        <Stylesheet id="leptos" href="/pkg/luxalpa_com.css"/>

        <Title text="Luxalpa's Lair"/>

        <Router>
            <Navigation />
            <main>
                <Routes fallback=move || "Not found." transition=true>
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
