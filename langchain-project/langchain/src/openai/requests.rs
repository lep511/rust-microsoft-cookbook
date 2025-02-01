use reqwest::{Client, Response};
use log::{warn, error};
use async_stream::stream;
use futures::StreamExt;
use crate::openai::{
    OPENAI_BASE_URL, OPENAI_EMBED_URL, RETRY_BASE_DELAY,
    DEBUG_PRE, DEBUG_POST,
};
use crate::llmerror::OpenAIError;
use crate::openai::libs::{
    ChatRequest, EmbedRequest, ErrorResponse, ChatResponse,
};
use crate::openai::utils::print_pre;
use std::time::Duration;
use tokio::time::sleep;

pub async fn request_chat(
    request: &ChatRequest,
    api_key: &str,
    timeout: Duration,
    max_retries: u32,
) -> Result<String, OpenAIError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    
    print_pre(&request, DEBUG_PRE);

    // Serializes the request struct into a JSON byte vector
    let request_body = serde_json::to_vec(request)?;

    let mut response: Response = make_request(
        &client,
        OPENAI_BASE_URL,
        api_key, 
        &request_body, 
        timeout,
    ).await?;

    for attempt in 1..=max_retries {
        if response.status().is_success() || response.status().as_u16() == 401 {
            // 401 - Invalid Authentication
            break;
        }

        warn!("Server error (attempt {}/{}): {}", attempt, max_retries, response.status());

        sleep(RETRY_BASE_DELAY).await;
        
        response = make_request(
            &client,
            OPENAI_BASE_URL,
            api_key,
            &request_body,
            timeout,
        ).await?;
    }

    // Checks if the response status is not successful (i.e., not in the 200-299 range).
    if !response.status().is_success() {
        let openai_error: OpenAIError = manage_error(response).await;
        return Err(openai_error);
    }

    let response_data = response.json::<serde_json::Value>().await?;
    print_pre(&response_data, DEBUG_POST);
    
    let response_string = response_data.to_string();
    Ok(response_string)
}

pub async fn request_embed(
    request: &EmbedRequest,
    api_key: &str,
) -> Result<String, OpenAIError> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let response: serde_json::Value;
    
    print_pre(&request, DEBUG_PRE);

    response = client
        .post(OPENAI_EMBED_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(request)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    print_pre(&response, DEBUG_POST);

    let response_string = response.to_string();
    Ok(response_string)
}

pub fn strem_chat(
    api_key: String,
    request: ChatRequest,
) -> impl futures::Stream<Item = ChatResponse> {
    stream! {
        let client = Client::new();

        let response: Response = match client
            .post(OPENAI_BASE_URL)
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
                                if !json_part.contains("delta") {
                                    continue;
                                }
                            
                                match serde_json::from_str::<ChatResponse>(json_part) {
                                    Ok(stream_response) => {
                                        yield stream_response;
                                    },
                                    Err(e) => {
                                        println!("Error parsing chunk: {:?}", json_part);
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

pub async fn make_request(
    client: &Client,
    url: &str,
    api_key: &str,
    request_body: &[u8],
    timeout: Duration,
) -> Result<Response, reqwest::Error> {
    Ok(client
        .post(url)
        .timeout(timeout)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(request_body.to_vec())
        .send()
        .await?)
}

pub async fn manage_error(
    response: Response,
) -> OpenAIError {
    error!("Response code: {}", response.status());

    match response.json::<ErrorResponse>().await {
        Ok(error_detail) => {
            match error_detail.error.code.as_str() {
                "invalid_api_key" => OpenAIError::AuthenticationError(
                    error_detail.error.message
                ),
                _ => OpenAIError::GenericError {
                    code: error_detail.error.code,
                    message: error_detail.error.message,
                    detail: "ERROR-req-9822".to_string(),
                },
            }
        }
        Err(e) => {
            OpenAIError::GenericError {
                code: "None".to_string(),
                message: format!("Error: {}", e),
                detail: "ERROR-req-9823".to_string(),
            }
        }
    }
}