use leptos::{tracing::info, *};
use serde::{Deserialize, Serialize};

/// A struct that represents a repo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    /// name of the repo
    pub name: String,
    /// url of the repo
    pub url: String,
}
/// Renders the home page of your application.
#[component]
pub fn Repos(cx: Scope) -> impl IntoView {
    let repo_names: Resource<(), Vec<Repo>> = create_resource(
        cx,
        move || (),
        |_| async move {
            let repo = user_repos_names("shawnyu5".to_string()).await.unwrap();
            info!("Loading resource...");
            return repo;
        },
    );

    view! { cx,
        <h1>"Repos"</h1>
        <div>
        <Suspense fallback=move || view!{ cx, <p>"Loading..."</p>}>
            {
                move ||
                    repo_names
                    .read(cx)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|e| {
                        info!("{:?}", e);
                        view!{ cx, <p><a href={e.url}>{e.name}</a></p><br/>}
                    })
                .collect_view(cx)
            }
        </Suspense>
        </div>
    }
}

/// A server function that returns a list of the user's repos names
#[server(Repos, "/api")]
pub async fn user_repos_names(user: String) -> Result<Vec<Repo>, ServerFnError> {
    use backend::github::repos::*;
    let repos = match get_all_user_repos(&user).await {
        Ok(repos) => repos,
        Err(e) => return Err(ServerFnError::ServerError(format!("{:?}", e))),
    };
    return Ok(repos
        .iter()
        .map(|r| Repo {
            name: r.name.clone(),
            url: r.html_url.clone().unwrap().into(),
        })
        .collect());
}
