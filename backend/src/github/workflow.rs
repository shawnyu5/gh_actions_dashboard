use super::auth::personal_access_token;
use anyhow::Result;
use octocrab::{models::workflows::Run, models::workflows::WorkFlow, Octocrab};

/// list all workflow runs for a repo
///
/// * `repo_owner`: owner of the repository
/// * `repo_name`: name of the repository
/// * `per_page`: number of runs per page. Default is 50.
pub async fn get_all_workflow_runs(
    repo_owner: &str,
    repo_name: &str,
    per_page: Option<u8>,
) -> Result<Vec<Run>> {
    let octocrab = Octocrab::builder()
        .personal_token(personal_access_token())
        .build()
        .unwrap();
    let workflow_runs = octocrab
        .workflows(repo_owner, repo_name)
        .list_all_runs()
        .per_page(per_page.unwrap_or_else(|| 50))
        .send()
        .await?;

    return Ok(workflow_runs.into_iter().collect());
}

/// list all workflows for a repo
///
/// * `repo_owner`: the owner of the repository
/// * `repo_name`: the name of the repository
/// * `per_page`: number of workflows per page. Default is 50.
pub async fn list_repo_workflows(
    repo_owner: &str,
    repo_name: &str,
    per_page: Option<u8>,
) -> Result<Vec<WorkFlow>> {
    let octocrab = Octocrab::builder()
        .personal_token(personal_access_token())
        .build()
        .unwrap();

    let workflows = octocrab
        .workflows(repo_owner, repo_name)
        .list()
        .per_page(per_page.unwrap_or_else(|| 50))
        .send()
        .await?;

    return Ok(workflows.into_iter().collect());
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_get_all_workflow_runs() {
        let runs = super::get_all_workflow_runs("shawnyu5", "gh-ac", None)
            .await
            .unwrap();
        assert!(runs.len() > 0);
    }

    #[tokio::test]
    async fn test_list_all_workflows() {
        let runs = super::list_repo_workflows("shawnyu5", "gh-ac", None)
            .await
            .unwrap();
        assert!(runs.len() > 0);
    }
}
