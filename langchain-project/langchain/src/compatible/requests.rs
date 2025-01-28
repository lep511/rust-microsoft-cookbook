use reqwest::Client;
use crate::compatible::{DEBUG_PRE, DEBUG_POST};
use crate::llmerror::CompatibleChatError;
use crate::compatible::libs::{ChatRequest, ErrorResponse};
use crate::compatible::utils::print_pre;
use std::time::Duration;

pub async fn request_chat(
    url: &str,
    request: &ChatRequest,
    api_key: &str,
    timeout: u64,
    retry: i32,
) -> Result<serde_json::Value, CompatibleChatError> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    
    print_pre(&request, DEBUG_PRE);
    
    let response = client
        .post(url)
        .timeout(Duration::from_secs(timeout))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        if retry > 0 {
            let mut n_count = 0;
            while n_count < retry {
                n_count += 1;
                println!(
                    "Error found. Retry {}.",
                    n_count,
                );
                // Wait for 2 sec
                tokio::time::sleep(Duration::from_secs(2)).await;
                let response = client
                    .post(url)
                    .timeout(Duration::from_secs(timeout))
                    .header("Authorization", format!("Bearer {}", api_key))
                    .header("Content-Type", "application/json")
                    .json(request)
                    .send()
                    .await?;

                if response.status().is_success() {
                    let response_data = response.json::<serde_json::Value>().await?;
                    print_pre(&response_data, DEBUG_POST);

                    return Ok(response_data);
                }
            }
        }
        println!("Response code: {}", response.status());
        match response.json::<ErrorResponse>().await {
            Ok(error) => {
                return Err(CompatibleChatError::GenericError {
                    message: error.detail,
                    detail: "ERROR-req-9877".to_string(),
                });
            }
            Err(e) => {
                return Err(CompatibleChatError::GenericError {
                    message: format!("Error: {}", e),
                    detail: "ERROR-req-9876".to_string(),
                });
            }
        }
    }

    let response_data = response.json::<serde_json::Value>().await?;
    print_pre(&response_data, DEBUG_POST);

    Ok(response_data)
}