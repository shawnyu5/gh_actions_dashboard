use anyhow::Result;
use octocrab::models::Repository;
use reqwest::header::AUTHORIZATION;

use super::auth::personal_access_token;

/// get all repos for a user
///
/// * `user_name`: the user to get the repos of
pub async fn get_all_user_repos(user_name: &str) -> Result<Vec<Repository>> {
    // https://api.github.com/users/USERNAME/repos
    let client = reqwest::Client::builder().user_agent(user_name).build()?;

    let res = client
        .get(format!("https://api.github.com/users/{}/repos", user_name))
        .header(AUTHORIZATION, format!("Bearer {}", personal_access_token()))
        .send()
        .await?;

    let json = res.json::<Vec<Repository>>().await?;
    return Ok(json);
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_get_all_user_repos() {
        let repos = super::get_all_user_repos("shawnyu5").await.unwrap();
        assert!(repos.len() > 0);
    }
}
