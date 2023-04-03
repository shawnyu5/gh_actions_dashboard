mod github;
use crate::github::auth::app_auth_token;

#[tokio::main]
async fn main() {
    let repos = github::workflow::get_all_org_repos("gh-action-dashboard-test")
        .await
        .unwrap();

    repos.into_iter().for_each(|repo| println!("{}", repo.name));
}
