use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::info;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| {
        set_count.update(|count| *count += 1);
        spawn_local(async move {
            let repo = user_repos().await.unwrap();
            info!("{}", repo);
        });
    };

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[server(Repos, "/api")]
pub async fn user_repos() -> Result<String, ServerFnError> {
    use backend::github::repos::*;
    let repos = get_all_user_repos("shawnyu5").await.unwrap();
    return Ok(format!("{:?}", repos[0]));
}
