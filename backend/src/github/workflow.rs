use anyhow::Result;
use octocrab::{
    models::{workflows::Run, Repository},
    Octocrab,
};
use reqwest::header::AUTHORIZATION;
use super::auth::app_auth_token;


/// get all repos for a user
///
/// * `user_name`: the user to get the repos of
pub async fn get_all_user_repos(user_name: &str) -> Result<Vec<Repository>> {
    // https://api.github.com/users/USERNAME/repos
    let client = reqwest::Client::builder().user_agent(user_name).build()?;

    let res = client
        .get(format!("https://api.github.com/users/{}/repos", user_name))
        .header(
            AUTHORIZATION,
            format!("Bearer {}", app_auth_token().await.unwrap()),
        )
        .send()
        .await?;

    let json = res.json::<Vec<Repository>>().await?;
    return Ok(json);
}

/// list all workflow runs for a repo
///
/// * `repo`: the repository to list runs for
pub async fn get_all_workflow_runs(repo_owner: &str, repo_name: &str) -> Result<Vec<Run>> {
    let octocrab = Octocrab::builder()
        .personal_token(app_auth_token().await.unwrap())
        .build()
        .unwrap();
    let workflow_runs = octocrab
        .workflows(repo_owner, repo_name)
        .list_all_runs()
        .send()
        .await?;

    return Ok(workflow_runs.into_iter().collect());
}
