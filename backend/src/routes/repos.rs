use octocrab::models::Repository;
use rocket::serde::json::Json;

use crate::github::repos::get_all_user_repos;

#[get("/repos")]
/// return all repos for a user
pub async fn user_repo() -> Result<Json<Vec<Repository>>, String> {
    let repos = match get_all_user_repos("shawnyu5").await {
        Ok(repos) => repos,
        Err(err) => return Err(err.to_string()),
    };
    return Ok(Json(repos));
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rocket;
    use rocket::http::Status;

    #[rocket::async_test]
    /// make sure we get at least 1 repo back
    async fn test_user_repo() {
        use rocket::local::asynchronous::Client;
        let client = Client::tracked(rocket()).await.unwrap();
        let response = client.get(uri!("/user", user_repo())).dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        assert_ne!(
            response
                .into_json::<Vec<Repository>>()
                .await
                .expect("unable to parse response into json")
                .len(),
            0
        );
    }
}
