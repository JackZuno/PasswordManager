use oauth2::{
    basic::BasicClient,
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::Client; 
use serde::Deserialize;
use tokio::sync::mpsc::Receiver;
use colored::*;


use crate::get_env_variable;

/// Struct to parse Google user info
#[derive(Deserialize)]
pub struct GoogleUserInfo {
    pub email: String,
    pub name: String,
}

/// Function to log in using Google OAuth
pub async fn login_with_google(
    mut rx: Receiver<String>,
) -> Result<Option<GoogleUserInfo>, Box<dyn std::error::Error + Send + Sync>> {
    let id_client = get_env_variable("ID_CLIENT");
    let client_secret = get_env_variable("CLIENT_SECRET");

    // Configure Google OAuth client
    let client = BasicClient::new(
        ClientId::new(id_client.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())?,
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080/callback".to_string())?);

    // Generate the authorization URL
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    println!("\nOpen this URL in your browser:\n{}", auth_url);

    // Wait for the authorization code after the listener is triggered
    let auth_code = rx.recv().await.unwrap();

    let message = "Authorization Code Received!".green().bold();
    println!("\n{}", message);

    // Exchange the authorization code for an access token
    let token_response = client
        .exchange_code(AuthorizationCode::new(auth_code))
        .request_async(async_http_client)
        .await?;

    let access_token = token_response.access_token().secret();

    // Fetch user info from Google API
    let user_info_url = "https://www.googleapis.com/oauth2/v2/userinfo";
    let reqwest_client = Client::new();
    let response = reqwest_client
        .get(user_info_url)
        .bearer_auth(access_token)
        .send()
        .await?;

    if !response.status().is_success() {
        println!("Failed to fetch user info.");
        return Ok(None);
    }

    let user_info: GoogleUserInfo = response.json().await?;

    Ok(Some(user_info))
}
