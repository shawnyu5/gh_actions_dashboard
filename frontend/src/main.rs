use github_types::Repository;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/repos")]
    Repos,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowRun {
    pub id: i64,
    #[serde(rename = "workflow_id")]
    pub workflow_id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    #[serde(rename = "head_branch")]
    pub head_branch: String,
    #[serde(rename = "head_sha")]
    pub head_sha: String,
    #[serde(rename = "run_number")]
    pub run_number: i64,
    pub event: String,
    pub status: String,
    pub conclusion: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "jobs_url")]
    pub jobs_url: String,
    #[serde(rename = "logs_url")]
    pub logs_url: String,
    #[serde(rename = "check_suite_url")]
    pub check_suite_url: String,
    #[serde(rename = "artifacts_url")]
    pub artifacts_url: String,
    #[serde(rename = "cancel_url")]
    pub cancel_url: String,
    #[serde(rename = "rerun_url")]
    pub rerun_url: String,
    #[serde(rename = "workflow_url")]
    pub workflow_url: String,
    #[serde(rename = "head_commit")]
    pub head_commit: HeadCommit,
    pub repository: WorkflowRepository,
    #[serde(rename = "head_repository")]
    pub head_repository: HeadRepository,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadCommit {
    pub id: String,
    #[serde(rename = "tree_id")]
    pub tree_id: String,
    pub message: String,
    pub timestamp: String,
    pub author: Author,
    pub committer: Committer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Committer {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowRepository {
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub owner: Owner,
    pub private: bool,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "deployments_url")]
    pub deployments_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadRepository {
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub owner: Owner2,
    pub private: bool,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "deployments_url")]
    pub deployments_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner2 {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[function_component(Home)]
fn all_workflow_runs() -> Html {
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

#[function_component(Repos)]
fn all_repos() -> Html {
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
        Route::Home => {
            html! { <Home /> }
        }
        Route::Repos => html! { <Repos /> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Main>::new().render();
}
