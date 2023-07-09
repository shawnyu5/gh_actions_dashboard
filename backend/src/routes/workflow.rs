use crate::github::workflow::get_all_workflow_runs;
use octocrab::models::workflows::Run;
use rocket::serde::json::Json;

#[get("/workflow_runs/<repo_owner>/<repo_name>")]
pub async fn workflow_runs(repo_owner: &str, repo_name: &str) -> Result<Json<Vec<Run>>, String> {
    let repo = match get_all_workflow_runs(repo_owner, repo_name, None).await {
        Ok(repo) => Ok(Json(repo)),
        Err(err) => return Err(err.to_string()),
    };
    return repo;
}
