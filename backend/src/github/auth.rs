use github_app_auth::{GithubAuthParams, InstallationAccessToken};
use reqwest::header::HeaderValue;

/// get a auth token for a installed github app
// return a auth token
pub async fn app_auth_token() -> Option<String> {
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

    let auth_token = header.get("authorization");
    let header_value = HeaderValue::to_str(auth_token.unwrap());

    return match header_value {
        Ok(value) => Some(value.to_string().replace("token ", "")),
        Err(_) => None,
    };
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    /// test the app_auth_token function returns a token
    async fn test_app_auth_token() {
        let token = app_auth_token().await;
        assert!(token.is_some());
    }
}
