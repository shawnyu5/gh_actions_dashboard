use crate::github::org::get_all_org_repos;
use octocrab::models::Repository;
use rocket::serde::json::Json;

#[get("/repo/<org_name>")]
pub async fn all_org_repos(org_name: &str) -> Result<Json<Vec<Repository>>, String> {
    let repos = match get_all_org_repos(org_name).await {
        Ok(repos) => repos,
        Err(err) => return Err(err.to_string()),
    };

    // convert repos into a vector
    let repo_vec: Vec<Repository> = repos.into_iter().map(|r| r).collect();
    return Ok(Json(repo_vec));
}
