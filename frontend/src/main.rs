mod components;
mod environment;
use crate::components::workflow::workflow::WorkflowRuns;
use crate::components::workflow::workflow::WorkflowSuccessRate;
use components::repos::Repos;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/workflow/runs")]
    WorkflowRuns,
    #[at("/repos")]
    Repos,
}

#[function_component(Home)]
fn app() -> Html {
    html! {
        <>
        <NavBar />
        <h1>{ "Home" }</h1>
        </>
    }
}

#[function_component(Router)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component(NavBar)]
fn nav_bar() -> Html {
    let navigator = use_navigator().unwrap();

    let go_to_repo = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Repos));
        html! { <button {onclick} >{ "Repos" }</button> }
    };

    let go_to_workflow_runs = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::WorkflowRuns));
        html! { <button {onclick} >{ "Workflow Runs" }</button> }
    };

    html! {
    <>
    {go_to_repo}
    {go_to_workflow_runs}
    </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Repos => html! { <Repos /> },
        Route::WorkflowRuns => html! {
            <>
                <WorkflowRuns />
                <WorkflowSuccessRate />
            </>
        },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Router>::new().render();
}
