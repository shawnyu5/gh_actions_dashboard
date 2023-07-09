use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::routes::{repo::Repos, workflow_run::WorkflowRun};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/gh_actions_dashboard.css"/>
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/foundation-sites@6.7.5/dist/css/foundation.min.css" crossorigin="anonymous" />
        // <script src="https://cdn.jsdelivr.net/npm/foundation-sites@6.7.5/dist/js/foundation.min.js" crossorigin="anonymous"></script>

        // sets the document title
        <Title text="Github actions dashboard"/>

        <Router>
            <main>
                <Routes>
                    <Route path="/repos" view=|cx| view! { cx, <Repos/> }/>
                    <Route path="/repos/workflow-run/:owner/:name" view=|cx| view! { cx, <WorkflowRun/> }/>
                </Routes>
            </main>
        </Router>
    }
}
