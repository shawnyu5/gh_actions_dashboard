use anyhow::Result;
use octocrab::{models::Repository, Octocrab, Page};

use super::auth::personal_access_token;

/// get all repos for an org
///
/// * `org_name`: the org name
/// return: a list of repositories
pub async fn get_all_org_repos(org_name: &str) -> Result<Page<Repository>> {
    let token = personal_access_token();
    let octocrab = Octocrab::builder().personal_token(token).build().unwrap();

    return Ok(octocrab.orgs(org_name).list_repos().send().await?);
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_get_all_org_repos() {
        let repos = super::get_all_org_repos("rust-lang").await.unwrap();
        assert!(repos.items.len() > 0);
    }
}
