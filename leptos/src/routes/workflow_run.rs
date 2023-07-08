use leptos::{tracing::info, *};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub id: u64,
    pub name: String,
    pub run_number: i64,
    pub event: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
    pub url: String,
    pub html_url: String,
}

#[component]
pub fn WorkflowRun(cx: Scope, repo_owner: String, repo_name: String) -> impl IntoView {
    let (repo_owner, _) = create_signal(cx, repo_owner);
    let (repo_name, _) = create_signal(cx, repo_name);
    let fetch_runs: Resource<(), Vec<Run>> = create_resource(
        cx,
        move || (),
        move |_| async move {
            let runs = repo_workflow_run(repo_owner.get(), repo_name.get())
                .await
                .unwrap();
            dbg!(&runs.len());
            return runs;
        },
    );

    return view! {cx,
        <Suspense fallback=move || view!{ cx, <p>"Loading..."</p>}>
        <table>
            <tr>
                <th>"Name"</th>
                <th>"Run number"</th>
                <th>"Conclusion"</th>
            </tr>
            <For
            each=move || fetch_runs.read(cx).unwrap_or_default()
            key=move |r| r.id
            view=move|cx, run: Run| {
                view!{ cx,
                    <tr>
                        <td>{run.name}</td>
                        <td>{run.run_number}</td>
                        <td>{run.conclusion}</td>
                    </tr>
                }
            }
            />
        </table>
    </Suspense>
    };
    // return view! {cx, <p>"Loading..."</p>};
}

#[server(RepoWorkflowRuns, "/api")]
async fn repo_workflow_run(
    repo_owner: String,
    repo_name: String,
) -> Result<Vec<Run>, ServerFnError> {
    use backend::github::workflow::*;

    let runs = match get_all_workflow_runs(repo_owner.as_str(), repo_name.as_str()).await {
        Ok(runs) => runs,
        Err(e) => return Err(ServerFnError::ServerError(format!("{:?}", e))),
    };

    return Ok(runs
        .iter()
        .map(|r| Run {
            id: r.id.into_inner(),
            name: r.name.clone(),
            run_number: r.run_number,
            event: r.event.clone(),
            status: r.status.clone(),
            conclusion: r.conclusion.clone(),
            url: r.url.clone().into(),
            html_url: r.html_url.clone().into(),
        })
        .collect());
}
