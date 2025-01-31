use reqwest::{Client, Response};
use log::{info, error};
use crate::openai::{
    OPENAI_BASE_URL, OPENAI_EMBED_URL, RETRY_BASE_DELAY,
    DEBUG_PRE, DEBUG_POST,
};
use crate::llmerror::OpenAIError;
use crate::openai::libs::{ChatRequest, EmbedRequest, ErrorResponse};
use crate::openai::utils::print_pre;
use std::time::Duration;
use tokio::time::sleep;

pub async fn request_chat(
    request: &ChatRequest,
    api_key: &str,
    timeout: u64,
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
        if response.status().is_success() {
            break;
        }

        info!(
            "Retry {}/{}. Code error: {:?}", 
            attempt,
            max_retries,
            response.status()
        );

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
        error!("Response code: {}", response.status());
        match response.json::<ErrorResponse>().await {
            Ok(error_detail) => {
                return Err(OpenAIError::GenericError {
                    message: error_detail.error.message,
                    detail: "ERROR-req-9822".to_string(),
                });
            }
            Err(e) => {
                return Err(OpenAIError::GenericError {
                    message: format!("Error: {}", e),
                    detail: "ERROR-req-9823".to_string(),
                });
            }
        }
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