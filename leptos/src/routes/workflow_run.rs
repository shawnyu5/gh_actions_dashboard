use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

/// a workflow run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRun {
    pub id: u64,
    pub repo: Repo,
    pub name: String,
    pub run_number: i64,
    pub event: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
    pub url: String,
    pub html_url: String,
}

/// a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    pub id: u64,
    pub name: String,
    pub full_name: Option<String>,
    pub html_url: String,
}

// TODO: idk why this doesn't work
// #[derive(Params, Copy, Clone, Debug, PartialEq, Eq)]
// pub struct WorkflowRunParam {
// owner: String,
// name: String,
// }

#[component]
pub fn WorkflowRun(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let repo_owner = params
        .with(|params| params.get("owner").cloned())
        .expect("owner route parameter");
    let repo_name = params
        .with(|params| params.get("name").cloned())
        .expect("name route parameter");

    // let (repo_owner, _) = create_signal(cx, repo_owner);
    // let (repo_name, _) = create_signal(cx, repo_name);

    // fetch workflow runs for a repo
    // let fetch_runs: Resource<(), Vec<WorkflowRun>> = create_resource(
    // cx,
    // move || (),
    // move |_| async move {
    // let runs = repo_workflow_run(repo_owner.get(), repo_name.get(), None)
    // .await
    // .unwrap();
    // return runs;
    // },
    // );

    return view! {cx,
    <Await
        future=move |_| repo_workflow_run(repo_owner.clone(), repo_name.clone(), None) // a server fn that returns Result<Vec<WorkflowRun>, ServerFnError>
        bind:data
    >
        <table>
            <tr>
                <th>"Name"</th>
                <th>"Run number"</th>
                <th>"Conclusion"</th>
            </tr>
            {
                data.clone().unwrap_or_default().into_iter().map(|run| {
                    view!{ cx,
                        <tr>
                            <td><a href=&run.repo.html_url target="_blank">{&run.name}</a></td>
                            <td>{run.run_number}</td>
                            <td>{&run.conclusion.clone().unwrap_or_default()}</td>
                        </tr>
                    }
                }).collect_view(cx)
            }
            </table>
    </Await>

    };
}

/// server function to list all workflow runs for a repo
#[server(RepoWorkflowRuns, "/api")]
pub async fn repo_workflow_run(
    repo_owner: String,
    repo_name: String,
    per_page: Option<u8>,
) -> Result<Vec<WorkflowRun>, ServerFnError> {
    use backend::github::workflow::*;

    let runs = match list_workflow_runs_for_repo(repo_owner.as_str(), repo_name.as_str(), per_page)
        .await
    {
        Ok(runs) => runs,
        Err(e) => return Err(ServerFnError::ServerError(format!("{:?}", e))),
    };

    return Ok(runs
        .iter()
        .map(|r| {
            let repo = Repo {
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
