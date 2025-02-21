use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: u64,
}

pub async fn exchange_code_for_tokens(
    client: &Client,
    token_endpoint: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
    code: &str,
) -> Result<TokenResponse, Box<dyn Error>> {
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    let response = client
        .post(token_endpoint)
        .form(&params)
        .send()
        .await?
        .json::<TokenResponse>()
        .await?;

    Ok(response)
}

pub async fn make_authenticated_request(
    client: &Client,
    url: &str,
    access_token: &str,
) -> Result<(), Box<dyn Error>> {
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Body: {}", response.text().await?);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let token_endpoint = "https://provider.example.com/oauth2/token";
    let client_id = "your_client_id";
    let client_secret = "your_client_secret";
    let redirect_uri = "http://localhost:8080/callback";
    let code = "authorization_code_from_redirect";

    let tokens = exchange_code_for_tokens(
        &client,
        token_endpoint,
        client_id,
        client_secret,
        redirect_uri,
        code,
    ).await?;

    println!("Access Token: {}", tokens.access_token);

    // Use the Access Token for Authenticated Requests
    let protected_url = "https://api.example.com/protected-resource";
    let access_token = "your_access_token";

    make_authenticated_request(&client, protected_url, access_token).await?;

    Ok(())
}