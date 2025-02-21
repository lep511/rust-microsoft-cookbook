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