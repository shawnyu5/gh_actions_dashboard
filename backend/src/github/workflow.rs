use super::auth::personal_access_token;
use anyhow::Result;
use octocrab::{models::workflows::Run, models::workflows::WorkFlow, Octocrab};

/// list all workflow runs for a repo
///
/// * `repo_owner`: owner of the repository
/// * `repo_name`: name of the repository
/// * `per_page`: number of runs per page. Default is 50.
pub async fn list_workflow_runs_for_repo(
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

/// list all workflow definitions for a repo
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

/// list workflow runs for a particular workflow
///
/// * `owner`: owner of the repo
/// * `repo`: repo name
/// * `workflow_id`: id of the workflow
/// * `per_page`: number of results to return
pub async fn list_workflow_runs_for_workflow<'a>(
    owner: impl Into<String>,
    repo: impl Into<String>,
    workflow_id: u64,
    per_page: Option<u8>,
) -> Result<Vec<Run>> {
    let octocrab = Octocrab::builder()
        .personal_token(personal_access_token())
        .build()
        .unwrap();

    let workflow_runs = octocrab
        .workflows(owner, repo)
        .list_runs(workflow_id.to_string())
        .per_page(per_page.unwrap_or_else(|| 50))
        .send()
        .await?;

    return Ok(workflow_runs.into_iter().collect());
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_get_all_workflow_runs() {
        let runs = list_workflow_runs_for_repo("shawnyu5", "gh-ac", None)
            .await
            .unwrap();
        assert!(runs.len() > 0);
    }

    #[tokio::test]
    async fn test_list_all_workflows() {
        let workflows = list_repo_workflows("shawnyu5", "gh-ac", None)
            .await
            .unwrap();
        assert!(workflows.len() > 0);
    }
    #[tokio::test]
    async fn test_list_workflow_run_for_workflow() {
        let workflows = list_repo_workflows("shawnyu5", "gh-ac", None)
            .await
            .unwrap();

        let workflow_runs = list_workflow_runs_for_workflow(
            "shawnyu5",
            "gh-ac",
            workflows[0].id.into_inner(),
            None,
        )
        .await
        .unwrap();
        assert!(workflow_runs.len() > 0)
    }
}
