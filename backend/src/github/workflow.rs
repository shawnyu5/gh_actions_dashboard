use anyhow::Result;
use octocrab::{models::Repository, Octocrab, Page};
use reqwest::header::AUTHORIZATION;

use super::auth::app_auth_token;

pub async fn get_all_org_repos(org_name: &str) -> Result<Page<Repository>> {
    let token = app_auth_token().await.unwrap();
    let octocrab = Octocrab::builder().personal_token(token).build().unwrap();

    return Ok(octocrab.orgs(org_name).list_repos().send().await?);
}

/// get all repos for a user
///
/// * `user_name`: the user to get the repos of
pub async fn get_all_user_repos(user_name: &str) -> Result<Vec<Repository>> {
    // https://api.github.com/users/USERNAME/repos
    let query = vec![("type", "owner")];
    let client = reqwest::Client::builder().user_agent(user_name).build()?;
    println!("{:?}", app_auth_token().await.unwrap());

    let res = client
        .get(format!("https://api.github.com/users/{}/repos", user_name))
        .header(
            AUTHORIZATION,
            format!("Bearer {}", app_auth_token().await.unwrap()),
        )
        .query(&query)
        .send()
        .await?;

    println!("{:?}", &res.headers());
    let json = res.json::<Vec<Repository>>().await?;
    return Ok(json);
}
