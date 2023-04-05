use github_types::Repository;
use log::{debug, info};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

#[function_component(Home)]
fn home() -> Html {
    // let repos = use_state(|| Vec::<Repository>::new());
    // let mut repo_clone = repos.clone();

    let repos: UseAsyncHandle<Vec<Repository>, String> = use_async(async move {
        let response = reqwest_wasm::get("http://localhost:8000/")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let js_obj = JsValue::from(&response);
        info!("js_obj: {:?}", js_obj);

        return match serde_json::from_str(&response) {
            Ok(repos) => Ok(repos),
            Err(e) => Err(e.to_string()),
        };
    });

    {
        let repos = repos.clone();
        use_effect_with_deps(
            move |_| {
                repos.run();
                || ()
            },
            (),
        )
    }

    if let Some(repos) = &repos.data {
        if repos.is_empty() {
            html! {
                <div>
                    <p>{ "No repos..." }</p>
                </div>
            }
        } else {
            html! {
                <div>
                    <h1>{ "Repos" }</h1>
                    {
                        for repos.iter().map(|repo| {
                            html! { <p><a href={ repo.clone().url }>{ &repo.name }</a></p> }
                        })
                    }
                </div>
            }
        }
    } else {
        html! {
            <div>
                <p>{ "Loading..." }</p>
            </div>
        }
    }
}
#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Main>::new().render();
}
