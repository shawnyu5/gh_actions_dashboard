use crate::github::workflow::get_all_user_repos;
use octocrab::models::Repository;
use rocket::serde::json::Json;

#[get("/")]
/// return all repos for a user
pub async fn user_repo() -> Result<Json<Vec<Repository>>, String> {
    let repos = match get_all_user_repos("shawnyu5").await {
        Ok(repos) => repos,
        Err(err) => return Err(err.to_string()),
    };
    return Ok(Json(repos));
}
