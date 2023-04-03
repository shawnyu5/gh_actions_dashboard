mod github;
use crate::github::auth::auth::app_auth_token;

#[tokio::main]
async fn main() {
    // let res = github::workflow::get_all_repos().await;
    // println!("{}", res);
    // yew::Renderer::<App>::new().render();
    let _ = app_auth_token().await;
    github::workflow::get_all_org_repos("gh-action-dashboard-test")
        .await
        .unwrap();
}
