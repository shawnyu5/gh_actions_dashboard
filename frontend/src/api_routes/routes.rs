use crate::environment::enviroment::ENVIRONMENT;

/// corresponds to /user/repos route
pub fn user_repos() -> String {
    return format!("{}/user/repos", &ENVIRONMENT.api_address);
}

/// corresponds to /user/workflow_runs/{repo_owner}/{repo_name} route
///
/// * `repo_owner`: owner of the repo
/// * `repo_name`: repo name
pub fn user_workflow_runs(repo_owner: String, repo_name: String) -> String {
    return format!(
        "{}/user/workflow_runs/{}/{}",
        &ENVIRONMENT.api_address, repo_owner, repo_name
        );
}
