use reqwest::Client;
use reqwest::{self, header::{HeaderMap, HeaderValue}};
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
    prefer: Option<&str>,
) -> Result<serde_json::Value, CompatibleChatError> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    
    let api_key_format = format!("Bearer {}", api_key);

    let api_key_header: HeaderValue = match HeaderValue::from_str(&api_key_format) {
        Ok(value) => value,
        Err(_) => return Err(CompatibleChatError::GenericError {
            message: "Invalid API key".to_string(),
            detail: "ERROR-req-9876".to_string(),
        }),
    };

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", api_key_header);
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    if let Some(prefer_value) = prefer {
        let prefer_header: HeaderValue = match HeaderValue::from_str(prefer_value) {
            Ok(value) => value,
            Err(_) => return Err(CompatibleChatError::GenericError {
                message: "Invalid prefer header value".to_string(),
                detail: "ERROR-req-9875".to_string(),
            }),
        };
        headers.insert("Prefer", prefer_header);
    }

    print_pre(&request, DEBUG_PRE);

    let response = client
        .post(url)
        .timeout(Duration::from_secs(timeout))
        .headers(headers)
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
        let error_response = response.json::<ErrorResponse>().await?;
        return Err(CompatibleChatError::GenericError {
            message: error_response.detail,
            detail: "ERROR-req-9877".to_string(),
        });
    }

    let response_data = response.json::<serde_json::Value>().await?;
    print_pre(&response_data, DEBUG_POST);

    Ok(response_data)
}