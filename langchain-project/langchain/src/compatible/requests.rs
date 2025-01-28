use reqwest::{Client, Response};
use async_stream::stream;
use futures::StreamExt;
use crate::compatible::{DEBUG_PRE, DEBUG_POST};
use crate::llmerror::CompatibleChatError;
use crate::compatible::libs::{ChatRequest, ErrorResponse, ChatResponse};
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
                    println!("Error sending request: {}", e);
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
                                        println!("Error parsing chunk: {}", e);
                                    }
                                }    
                            }
                        }
                    },
                    Err(e) => {
                        println!("Error reading chunk: {}", e);
                    }
                }
            }
        }
    }
}