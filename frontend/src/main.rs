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
        <h1>{ "Home" }</h1>
        </>
    }
}

#[function_component(Router)]
fn app() -> Html {
    html! {
            <BrowserRouter>
                <NavBar />
                <Switch<Route> render={switch} />
            </BrowserRouter>
    }
}

#[function_component(NavBar)]
fn nav_bar() -> Html {
    html! {
    <nav class="navbar navbar-expand-lg bg-body-tertiary">
      <div class="container-fluid">
        <a class="navbar-brand">{"Github dashboard"}</a>
        <button
          class="navbar-toggler"
          type="button"
          data-bs-toggle="collapse"
          data-bs-target="#navbarNavAltMarkup"
          aria-controls="navbarNavAltMarkup"
          aria-expanded="false"
          aria-label="Toggle navigation"
        >
          <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarNavAltMarkup">
          <div class="navbar-nav">
            <a class="nav-link active">
              <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            </a>
            <a class="nav-link">
              <Link<Route> to={Route::Repos}>{ "Repos" }</Link<Route>>
            </a>
            <a class="nav-link">
              <Link<Route> to={Route::WorkflowRuns}>{ "Workflow runs" }</Link<Route>>
            </a>
          </div>
        </div>
      </div>
    </nav>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Repos => html! {
            <Repos />
        },
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
