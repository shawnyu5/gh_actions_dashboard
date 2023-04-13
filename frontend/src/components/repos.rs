use crate::api_routes::routes::user_repos;
use github_types::Repository;
use log::info;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// get the user repo's parsed into text
async fn get_user_repos_text() -> Result<String, reqwest_wasm::Error> {
    return Ok(
        reqwest_wasm::get(user_repos())
            .await?
            .text()
            .await?,
    );
}
#[function_component(Repos)]
pub fn all_repos() -> Html {
    let loading_state = use_state(|| false);

    let repos: UseAsyncHandle<Vec<Repository>, String> = {
        let loading_state = loading_state.clone();
        use_async(async move {
            loading_state.set(true);
            let response = match get_user_repos_text().await {
                Ok(repos) => repos,
                Err(e) => {
                    loading_state.set(false);
                    return Err(e.to_string());
                }
            };
            let js_obj = JsValue::from(&response);
            info!("{}", js_obj.as_string().unwrap());

            loading_state.set(false);
            return match serde_json::from_str(&response) {
                Ok(repos) => Ok(repos),
                Err(e) => Err(e.to_string()),
            };
        })
    };

    {
        let repos = repos.clone();
        use_effect_with_deps(
            move |_| {
                repos.run();
            },
            (),
        )
    }

    if *loading_state {
        html! {
            <div>
                <p>{ "Loading..." }</p>
            </div>
        }
    } else if let Some(repos) = &repos.data {
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
                <p>{ "Error getting repos" }</p>
            </div>
        }
    }
}
