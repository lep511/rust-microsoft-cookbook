use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub async fn get_auth_endpoint(iss: &str) -> Result<String, Error> {
    let client = Client::new();
    let url = format!("{}/.well-known/smart-configuration", iss);
    let response = client.get(&url).send().await?.json().await?;

    let auth_endpoint: String = response["authorization_endpoint"]
        .as_str()
        .unwrap()
        .to_string();

    Ok(auth_endpoint)
}