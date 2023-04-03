use octocrab::{models::Repository, Octocrab, Page};

use super::auth::auth::app_auth_token;

pub async fn get_all_org_repos(org_name: &str) -> Result<Page<Repository>, octocrab::Error> {
    // let octocrab = octocrab::instance();
    let token = app_auth_token().await.unwrap();
    let octocrab = Octocrab::builder().personal_token(token).build().unwrap();

    return Ok(octocrab.orgs(org_name).list_repos().send().await?);
}
