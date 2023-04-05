use crate::components::workflow::types::WorkflowRun;
use github_types::Repository;
use log::debug;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(WorkflowRuns)]
pub fn all_workflow_runs() -> Html {
    let _ = use_navigator().unwrap();
    // get all repos for a user
    let repos = async {
        let response = reqwest_wasm::get("http://localhost:8000/")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        return match serde_json::from_str(&response) {
            Ok(repos) => Ok(repos),
            Err(e) => Err(e.to_string()),
        };
    };

    // get all workflow runs for all repos of a user
    let workflow_runs_handler: UseAsyncHandle<Vec<WorkflowRun>, String> = use_async(async move {
        let mut workflow_runs: Vec<WorkflowRun> = Vec::new();
        let repos: Vec<Repository> = repos.await.unwrap();
        for repo in repos {
            let response = reqwest_wasm::get(&format!(
                "http://localhost:8000/workflow_runs/{}/{}",
                repo.owner.login, repo.name
            ))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

            // let json: WorkflowRun = serde_json::from_str(&response).unwrap();
            // debug!("{:?}", JsValue::from(&response));
            // let json: Vec<WorkflowRun> = serde_json::from_str(&response).unwrap();

            if let Ok(mut json) = serde_json::from_str::<Vec<WorkflowRun>>(&response) {
                debug!("{:?}", JsValue::from(&response));
                workflow_runs.append(&mut json);
            } else {
                debug!(
                    "No workflow runs found for {:?}/{:?}",
                    JsValue::from(repo.owner.login),
                    JsValue::from(repo.name)
                );
            }
        }
        return Ok(workflow_runs);
    });

    {
        let workflow_runs_handler = workflow_runs_handler.clone();
        use_effect_with_deps(
            move |_| {
                workflow_runs_handler.run();
                || ()
            },
            (),
        );
    }

    if let Some(workflow) = &workflow_runs_handler.data {
        if workflow.len() == 0 {
            html! {<div>{"No workflow runs..."}</div>}
        } else {
            html! {
                <div>
                    <tr>
                        <th>{"Repository"}</th>
                        <th>{"Job name"}</th>
                        <th>{"Status"}</th>
                    </tr>
                {
                    for workflow.iter().map(|w| {
                        html! {
                            <tr>
                                <td>{&w.repository.name}</td>
                                <td>{&w.name}</td>
                                <td>{&w.conclusion}</td>
                            </tr>
                        }
                    })
                }
                </div>
            }
        }
    } else {
        html! {<div>{"Loading..."}</div>}
    }
}
