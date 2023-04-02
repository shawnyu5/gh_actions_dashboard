use github_app_auth::{GithubAuthParams, InstallationAccessToken};

/// get a auth token for a installed github app
async fn gh_auth_token() {
    let pem_file =
        std::fs::read_to_string("rust-gh-action-dashboard.2023-04-01.private-key.pem").unwrap();

    // The token is mutable because the installation access token must be
    // periodically refreshed. See the `GithubAuthParams` documentation
    // for details on how to get the private key and the two IDs.
    let mut token = InstallationAccessToken::new(GithubAuthParams {
        user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36".into(),
        private_key: pem_file.into(),
        app_id: 312828,
        installation_id: 35962623,
    })
    .await
    .expect("failed to get installation access token");

    let header = token
        .header()
        .await
        .expect("failed to get authentication header");

    println!("header: {:?}", header);
}
async fn octocrab_app_auth() {
    let app_id = AppId(312828);
    let pem_file =
        std::fs::read_to_string("rust-gh-action-dashboard.2023-04-01.private-key.pem").unwrap();
    let pem_key = EncodingKey::from_rsa_pem(pem_file.as_bytes()).unwrap();

    let crab = Octocrab::builder().app(app_id, pem_key).build().unwrap();

    let user = crab.current();
    println!("Hello, {:?}!", user.user().await.unwrap());
}

#[tokio::main]
async fn main() {
    // yew::Renderer::<App>::new().render();
    gh_auth_token().await;
    octocrab_app_auth().await;
    // get_installation_id();
}
