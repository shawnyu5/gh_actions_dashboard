use super::auth::personal_access_token;
use anyhow::Result;
use octocrab::{models::workflows::Run, Octocrab};

/// list all workflow runs for a repo
///
/// * `repo`: the repository to list runs for
pub async fn get_all_workflow_runs(repo_owner: &str, repo_name: &str) -> Result<Vec<Run>> {
    let octocrab = Octocrab::builder()
        .personal_token(personal_access_token())
        .build()
        .unwrap();
    let workflow_runs = octocrab
        .workflows(repo_owner, repo_name)
        .list_all_runs()
        .send()
        .await?;

    return Ok(workflow_runs.into_iter().collect());
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_get_all_workflow_runs() {
        let runs = super::get_all_workflow_runs("shawnyu5", "gh-ac")
            .await
            .unwrap();
        assert!(runs.len() > 0);
    }
}
