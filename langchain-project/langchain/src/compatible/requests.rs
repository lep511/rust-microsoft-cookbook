use reqwest::{Client, Response};
use log::{warn, error};
use async_stream::stream;
use futures::StreamExt;
use crate::compatible::{DEBUG_PRE, DEBUG_POST, RETRY_BASE_DELAY};
use crate::llmerror::CompatibleChatError;
use crate::compatible::libs::{ChatRequest, ErrorResponse, ChatResponse};
use crate::compatible::utils::print_pre;
use std::time::Duration;
use serde_json::Value;
use tokio::time::sleep;

pub async fn request_chat(
    url: &str,
    request: &ChatRequest,
    api_key: &str,
    timeout: u64,
    max_retries: i32,
) -> Result<serde_json::Value, CompatibleChatError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    
    print_pre(&request, DEBUG_PRE);
    
    // Serializes the request struct into a JSON byte vector
    let request_body = serde_json::to_vec(request)?;

    let mut response: Response = make_request(
        &client,
        url,
        api_key, 
        &request_body, 
        timeout,
    ).await?;

    for attempt in 1..=max_retries {
        if response.status().is_success() { break; }

        warn!("Server error (attempt {}/{}): {}", attempt, max_retries, response.status());

        sleep(RETRY_BASE_DELAY).await;
        
        response = make_request(
            &client,
            url,
            api_key,
            &request_body,
            timeout,
        ).await?;
    }

    // Checks if the response status is not successful (i.e., not in the 200-299 range).
    if !response.status().is_success() {
        let comp_error: CompatibleChatError = manage_error(response).await;
        return Err(comp_error);
    }

    let response_data = response.json::<serde_json::Value>().await?;
    print_pre(&response_data, DEBUG_POST);

    Ok(response_data)
}

pub async fn get_request(
    url: &str, 
    api_key: &str
) -> Result<Value, CompatibleChatError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let response = client
        .request(reqwest::Method::GET, url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Accept", "application/json")
        .send()
        .await?;

    if !response.status().is_success() {
        let comp_error: CompatibleChatError = manage_error(response).await;
        return Err(comp_error);
    }
    
    let response_data = response.json::<serde_json::Value>().await?;

    print_pre(&response_data, DEBUG_POST);

    Ok(response_data)
}

pub fn strem_chat(
    url: String,
    api_key: String,
    request: ChatRequest,
) -> impl futures::Stream<Item = ChatResponse> {
    stream! {
        let client = Client::new();

        let response: Response = match client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await {
                Ok(response) => response,
                Err(e) => {
                    error!("Error Error sending request: {}", e);
                    return
                }
            };

        if response.status().is_success() {
            let mut stream = response.bytes_stream();

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        let str_chunk = String::from_utf8_lossy(&bytes);
                        let parts: Vec<&str> = str_chunk.split("\n\n").collect();
                        for part in parts {
                            if !part.is_empty() && part.ends_with("[DONE]") {
                                break;
                            }

                            if !part.is_empty() && part.starts_with("data:") {
                                let json_part = part.trim_start_matches("data:");
                            
                                match serde_json::from_str::<ChatResponse>(json_part) {
                                    Ok(stream_response) => {
                                        yield stream_response;
                                    },
                                    Err(e) => {
                                        warn!("Error Error parsing chunk: {}", e);
                                    }
                                }    
                            }
                        }
                    },
                    Err(e) => {
                        warn!("Error Error reading chunk: {}", e);
                    }
                }
            }
        } else {
            error!("Error Request failed with status code: {}", response.status());
        }
    }
}

/// Makes an HTTP POST request with the provided parameters
/// 
/// # Arguments
/// 
/// * `client` - Reference to a reqwest Client instance for making HTTP requests
/// * `url` - The target URL endpoint for the POST request
/// * `api_key` - API key used for Bearer token authentication
/// * `request_body` - Byte slice containing the JSON request body
/// * `timeout` - Request timeout duration in seconds
/// 
/// # Returns
/// 
/// * `Result<Response, reqwest::Error>` - Returns the HTTP response if successful,
///   or a reqwest error if the request fails
///
/// # Errors
///
/// Returns a `reqwest::Error` if:
/// * The request fails to send
/// * The connection times out
/// * There are network-related issues
///
pub async fn make_request(
    client: &Client,
    url: &str,
    api_key: &str,
    request_body: &[u8],
    timeout: u64,
) -> Result<Response, reqwest::Error> {
    Ok(client
        .post(url)
        .timeout(Duration::from_secs(timeout))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(request_body.to_vec())
        .send()
        .await?)
}

pub async fn manage_error(
    response: Response,
) -> CompatibleChatError {
    error!("Response code: {}", response.status());

    match response.json::<ErrorResponse>().await {
        Ok(error) => {
            if let Some(error) = error.error {
                CompatibleChatError::GenericError {
                    message: error.message,
                    detail: "ERROR-req-9821".to_string(),
                }
            } else if let Some(error) = error.detail {
                CompatibleChatError::GenericError {
                    message: error,
                    detail: "ERROR-req-9822".to_string(),
                }
            } else {
                CompatibleChatError::GenericError {
                    message: "Unknown error.".to_string(),
                    detail: "ERROR-req-9823".to_string(),
                }
            }
        },
        Err(_) => {
            CompatibleChatError::GenericError {
                message: "Unknown error.".to_string(),
                detail: "ERROR-req-9824".to_string(),
            }
        }
    }
}