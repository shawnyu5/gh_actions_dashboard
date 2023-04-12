use anyhow::Result;
use octocrab::{
    models::Repository,
    Octocrab, Page,
};
use super::auth::app_auth_token;

/// get all repos for an org
///
/// * `org_name`: the org name
/// return: a list of repositories
pub async fn get_all_org_repos(org_name: &str) -> Result<Page<Repository>> {
    let token = app_auth_token().await.unwrap();
    let octocrab = Octocrab::builder().personal_token(token).build().unwrap();

    return Ok(octocrab.orgs(org_name).list_repos().send().await?);
}
