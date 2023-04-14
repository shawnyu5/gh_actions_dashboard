use super::types::WorkflowRunConclusion;
use crate::api_routes::routes::{user_repos, user_workflow_runs};
use crate::components::{counter::counter::Counter, workflow::types::WorkflowRun};
use cached::proc_macro::cached;
use github_types::Repository;
use log::info;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// get all workflow runs with a status
///
/// * workflow_runs: the workflow runs to filter
fn with_status(workflow_runs: &Vec<WorkflowRun>, conclusion: WorkflowRunConclusion) -> Vec<WorkflowRun> {
    return workflow_runs
        .to_owned()
        .into_iter()
        .filter(|w| w.conclusion == conclusion)
        .collect();
}

/// fetch all the user's repos
/// returns the user's repos
async fn get_user_repos() -> Result<String, reqwest_wasm::Error> {
    return Ok(reqwest_wasm::get(user_repos()).await?.text().await?);
}

/// get all workflow runs for a repository
///
/// * `repo`: the repository to get workflow runs of
/// * return: the workflow runs of the repository
#[cached(
    // cache for 10 mins
    time = 600,
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

    let response = reqwest_wasm::get(user_workflow_runs(repo.owner.login, repo.name))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return serde_json::from_str::<Vec<WorkflowRun>>(&response).ok();
}

/// calculate the success rate of workflow runs
///
/// * `workflow_runs`: the workflow run
/// returns the number of successful runs, the number of failed runs, and the success rate
fn calculate_success_rate(workflow_runs: &Vec<WorkflowRun>) -> (usize, usize, f32) {
    let success_run_len = with_status(&workflow_runs, WorkflowRunConclusion::success).len();
    let failure_run_len = with_status(&workflow_runs, WorkflowRunConclusion::failure).len();
    let success_percent =
        success_run_len as f32 / (success_run_len + failure_run_len) as f32 * 100 as f32;

    return (success_run_len, failure_run_len, success_percent);
}

#[function_component(WorkflowRuns)]
/// get all the workflow runs for a user's repositories
pub fn all_workflow_runs() -> Html {
    let is_loading_state = use_state(|| false);
    let success_run_state = use_state(|| 0.0 as usize);
    let failed_runs_state = use_state(|| 0 as usize);
    let success_rate_state = use_state(|| 0.0 as f32);

    // get all workflow runs for a repo
    let workflow_runs_handler: UseAsyncHandle<Vec<WorkflowRun>, String> = {
        let is_loading_state = is_loading_state.clone();
        let success_run_state = success_run_state.clone();
        let failed_runs_state = failed_runs_state.clone();
        let success_rate_state = success_rate_state.clone();

        use_async(async move {
            is_loading_state.set(true);
            // contain all workflow runs
            let mut workflow_runs: Vec<WorkflowRun> = Vec::new();
            // get all repos for the current user
            let repos = match get_user_repos().await {
                Ok(repos) => repos,
                Err(e) => {
                    return {
                        is_loading_state.set(false);
                        Err(e.to_string())
                    }
                }
            };
            let repos: Option<Vec<Repository>> = match serde_json::from_str(&repos) {
                Ok(repos) => Some(repos),
                Err(_) => None,
            };

            if repos.is_none() {
                is_loading_state.set(false);
                return Err("Error getting repos".to_string());
            }

            for repo in repos.unwrap() {
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
            let (success_runs, failed_runs, success_rate) = calculate_success_rate(&workflow_runs);

            success_run_state.set(success_runs);
            failed_runs_state.set(failed_runs);
            success_rate_state.set(success_rate);

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
        html! { <Counter title="Time passed: " increment=1 /> }
    } else if let Some(workflow) = &workflow_runs_handler.data {
        html! {
            <>
                <div id="workflow-runs">
                    <table class="table">
                    <tr>
                        <th>{"Repository"}</th>
                        <th>{"Job name"}</th>
                        <th>{"Status"}</th>
                    </tr>
                    {
                        for workflow.iter().map(|w| {
                            let table_class = {
                                if w.conclusion == WorkflowRunConclusion::success {
                                    "background-color: #73ff85"
                                } else if w.conclusion == WorkflowRunConclusion::failure {
                                    "background-color: #f56969"
                                } else if w.conclusion == WorkflowRunConclusion::cancelled {
                                    "background-color: #f5f25b"
                                } else {
                                    ""
                                }
                            };
                            html! {
                                <tr style={table_class}>
                                    <td><a href={w.clone().repository.html_url}>{&w.repository.name}</a></td>
                                    <td>{&w.name}</td>
                                    <td>{&w.conclusion}</td>
                                </tr>
                            }
                        })
                    }
                    </table>
                </div>
                <div id="workflow-success-rate">
                    <p>{"Success: "}{*success_run_state}</p>
                    <p>{"failed: "}{*failed_runs_state}</p>
                    <p>{"Success rate: "}{*success_rate_state}{"%"}</p>
                </div>
            </>
        }
    } else {
        html! {<div>{"Error loading all workflows..."}</div>}
    }
}
