mod github;
use crate::github::auth::auth::app_auth_token;

#[tokio::main]
async fn main() {
    // yew::Renderer::<App>::new().render();
    let token = app_auth_token().await;
}
