use crate::environment::enviroment::ENVIRONMENT;
use github_types::Repository;
use log::info;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component(Repos)]
pub fn all_repos() -> Html {
    let repos: UseAsyncHandle<Vec<Repository>, String> = use_async(async move {
        let response = reqwest_wasm::get(format!("{}/user/repos", &ENVIRONMENT.api_address))
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
                            html! { <p><a href={ repo.clone().html_url }>{ &repo.name }</a></p> }
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
