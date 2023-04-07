use super::types::WorkflowRunConclusion;
use crate::components::workflow::types::WorkflowRun;
use cached::proc_macro::cached;
use github_types::Repository;
use log::{debug, info};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// get all workflow runs with a status
///
/// * workflow_runs: the workflow runs to filter
/// * conclusion: the target conclusion state
/// * return: the filtered workflow runs
fn status(workflow_runs: &Vec<WorkflowRun>, conclusion: WorkflowRunConclusion) -> Vec<WorkflowRun> {
    return workflow_runs
        .to_owned()
        .into_iter()
        .filter(|w| w.conclusion == conclusion)
        .collect();
}

/// get all workflow runs for a repository
///
/// * `repo`: the repository to get workflow runs of
/// * return: the workflow runs of the repository
#[cached(
    time = 200,
    option = true,
    sync_writes = true,
    key = "String",
    convert = "{repo.clone().name}"
)]
async fn get_repo_workflow_runs(repo: Repository) -> Option<Vec<WorkflowRun>> {
    info!(
        "Getting workflow runs for {:?}/{:?}",
        JsValue::from(&repo.owner.login).as_string().unwrap(),
        JsValue::from(&repo.name).as_string().unwrap()
    );

    let response = reqwest_wasm::get(&format!(
        "http://localhost:8000/workflow_runs/{}/{}",
        repo.owner.login, repo.name
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
    return serde_json::from_str::<Vec<WorkflowRun>>(&response).ok();
}

#[function_component(WorkflowRuns)]
/// get all the workflow runs for a user's repositories
pub fn all_workflow_runs() -> Html {
    let is_loading_state = use_state(|| false);

    // get all repos for a user
    let get_repos = async {
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

    let is_loading_state = is_loading_state.clone();
    // get the number of success and failed runs
    let workflow_runs_handler: UseAsyncHandle<Vec<WorkflowRun>, String> = {
        let is_loading_state = is_loading_state.clone();
        use_async(async move {
            is_loading_state.set(true);
            // contain all workflow runs
            let mut workflow_runs: Vec<WorkflowRun> = Vec::new();
            // get all repos for the current user
            let repos: Vec<Repository> = get_repos.await.unwrap();

            for repo in repos {
                if let Some(mut runs) = get_repo_workflow_runs(repo.clone()).await {
                    workflow_runs.append(&mut runs);
                } else {
                    info!(
                        "Error getting workflow runs found for {:?}/{:?}",
                        JsValue::from(repo.owner.login).as_string().unwrap(),
                        JsValue::from(repo.name).as_string().unwrap()
                    );
                };
            }

            is_loading_state.set(false);
            return Ok(workflow_runs);
        })
    };

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
    if *is_loading_state {
        html! {<div>{"Loading..."}</div>}
    } else if let Some(workflow) = &workflow_runs_handler.data {
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
                                <td><a href={w.clone().repository.html_url}>{&w.repository.name}</a></td>
                                <td>{&w.name}</td>
                                <td>{&w.conclusion}</td>
                            </tr>
                        }
                    })
                }
                </div>
        }
    } else {
        html! {<div>{"Error loading all workflows..."}</div>}
    }
}

#[function_component(WorkflowSuccessRate)]
/// get the total number of success and failed workflow runs, and calculate the success percentage
pub fn get_workflow_success_rate() -> Html {
    // get all repos for a user
    let get_repos = async {
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

    // get the number of success and failed runs
    let workflow_stats_handler: UseAsyncHandle<(usize, usize, f32), String> =
        use_async(async move {
            // contain all workflow runs
            let mut workflow_runs: Vec<WorkflowRun> = Vec::new();
            // get all repos for the current user
            let repos: Vec<Repository> = get_repos.await.unwrap();

            for repo in repos {
                if let Some(mut runs) = get_repo_workflow_runs(repo.clone()).await {
                    workflow_runs.append(&mut runs);
                } else {
                    debug!(
                        "No workflow runs found for {:?}/{:?}",
                        JsValue::from(repo.owner.login).as_string().unwrap(),
                        JsValue::from(repo.name).as_string().unwrap()
                    )
                };
            }
            let success_run_len = status(&workflow_runs, WorkflowRunConclusion::success).len();
            let failure_run_len = status(&workflow_runs, WorkflowRunConclusion::failure).len();
            let success_percent =
                success_run_len as f32 / (success_run_len + failure_run_len) as f32 * 100 as f32;

            return Ok((success_run_len, failure_run_len, success_percent));
        });

    {
        let workflow_stats_handler = workflow_stats_handler.clone();
        use_effect_with_deps(
            move |_| {
                workflow_stats_handler.run();
                || ()
            },
            (),
        );
    }

    if let Some(workflow) = &workflow_stats_handler.data {
        html! {
                <div>
                    <p>{"Success: "}{workflow.0}</p>
                    <p>{"failed: "}{workflow.1}</p>
                    <p>{"Success rate: "}{workflow.2}{"%"}</p>
                </div>
        }
    } else {
        html! {<div>{"Calculating..."}</div>}
    }
}
