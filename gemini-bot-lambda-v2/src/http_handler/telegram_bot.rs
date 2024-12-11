use reqwest::{ Client, Body };
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

pub async fn send_message(token: &str, chat_id: i32, text: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // https://api.telegram.org/bot7796241975:AAEnE3G8IaUhx-HydXlp5Yc0Fr8OQ0nHE3k/getUpdates
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let client = Client::new();
    
    // Prepare the request body
    let request_body = json!({
        "chat_id": chat_id,
        "text": text
    });
    
    let request_body = serde_json::to_string(&request_body)?;
    let body: Body = Body::wrap(request_body);
    
    // Send the POST request
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    // Print status code
    println!("Status: {}", response.status());
    
    let json = response.json().await?;
    Ok(json)
}