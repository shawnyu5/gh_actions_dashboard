use leptos::*;
use serde::{Deserialize, Serialize};

use crate::routes::repo::user_repos_names;

use super::workflow_run::WorkflowRun;

#[component]
pub fn home(cx: Scope) -> impl IntoView {
    // let params = use_params_map(cx);
    let user = "shawnyu5".to_string();
    // let user = params
    // .with(|params| params.get("user").cloned())
    // .expect("user route parameter");
    let (user, _) = create_signal(cx, user);

    view! { cx,
        <Await
            future=move |_| get_all_user_repo_workflow_runs(user.get())
            bind:data
            >
            <table>
                <tr>
                    <th>"Repo"</th>
                    <th>"Name"</th>
                    <th>"Status"</th>
                </tr>
                {
                    data.into_iter().map(|w|{
                        view! {cx,
                            <tr>
                                <td><a href=&w.repo.html_url target="_blank">{&w.repo.name}</a></td>
                                <td><a href=&w.html_url target="_blank">{&w.name}</a></td>
                                <td>{&w.status}</td>
                            </tr>
                        }
                    }).collect_view(cx)
                }
            </table>
        </Await>
    }
}

/// get the latest workflow run for all of a user's repos
async fn get_all_user_repo_workflow_runs(user: String) -> Vec<WorkflowRun> {
    let repos = user_repos_names(user.clone()).await.unwrap();
    let mut all_repo_workflow_runs: Vec<WorkflowRun> = vec![];

    // TODO: should try to multi thread this
    for repo in repos {
        let repo_workflows = list_repo_workflows(user.clone(), repo.name.clone(), Some(1))
            .await
            .unwrap();
        for workflow in repo_workflows {
            let workflow_run = list_workflow_runs_for_workflow(
                user.clone(),
                repo.name.clone(),
                workflow.id,
                Some(1),
            )
            .await
            .unwrap();
            all_repo_workflow_runs.extend(workflow_run);
        }
    }

    return all_repo_workflow_runs;
}

/// a single workflow definition
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepoWorkflow {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub state: String,
    pub html_url: String,
}

/// fetch a list of workflow definitions for a repo
#[server(ListRepoWorkFlows, "/api")]
async fn list_repo_workflows(
    owner: String,
    repo: String,
    per_page: Option<u8>,
) -> Result<Vec<RepoWorkflow>, ServerFnError> {
    use backend::github::workflow::*;
    let repo_workflows = match list_repo_workflows(owner.as_str(), repo.as_str(), per_page).await {
        Ok(workflows) => workflows,
        Err(e) => return Err(ServerFnError::ServerError(format!("{:?}", e))),
    };

    return Ok(repo_workflows
        .iter()
        .map(|w| RepoWorkflow {
            id: *w.id,
            name: w.name.clone(),
            path: w.path.clone(),
            state: w.state.clone(),
            html_url: w.html_url.clone().into(),
        })
        .collect());
}

/// fetch a list of workflow runs for a workflow
#[server(WorkflowRuns, "/api")]
async fn list_workflow_runs_for_workflow(
    owner: String,
    repo: String,
    workflow_id: u64,
    per_page: Option<u8>,
) -> Result<Vec<WorkflowRun>, ServerFnError> {
    use backend::github::workflow::list_workflow_runs_for_workflow;
    // list_workflow_runs_for_workflow
    let workflow_runs =
        match list_workflow_runs_for_workflow(owner, repo, workflow_id, per_page).await {
            Ok(runs) => runs,
            Err(e) => return Err(ServerFnError::ServerError(format!("{:?}", e))),
        };

    return Ok(workflow_runs
        .iter()
        .map(|r| {
            let repo = super::workflow_run::Repo {
                id: r.repository.id.into_inner(),
                name: r.repository.name.clone(),
                full_name: r.repository.full_name.clone(),
                html_url: r.repository.html_url.clone().unwrap().into(),
            };

            WorkflowRun {
                id: r.id.into_inner(),
                repo,
                name: r.name.clone(),
                run_number: r.run_number,
                event: r.event.clone(),
                status: r.status.clone(),
                conclusion: r.conclusion.clone(),
                url: r.url.clone().into(),
                html_url: r.html_url.clone().into(),
            }
        })
        .collect());
}
