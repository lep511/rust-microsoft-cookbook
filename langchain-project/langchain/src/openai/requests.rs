use reqwest::Client;
use crate::openai::{OPENAI_BASE_URL, OPENAI_EMBED_URL};
use crate::openai::{DEBUG_PRE, DEBUG_POST};
use crate::llmerror::OpenAIError;
use crate::openai::libs::{ChatRequest, EmbedRequest};
use crate::openai::utils::print_pre;
use std::time::Duration;

pub async fn request_chat(
    request: &ChatRequest,
    api_key: &str,
    timeout: u64,
    retry: i32,
) -> Result<String, OpenAIError> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let mut response: serde_json::Value;
    
    print_pre(&request, DEBUG_PRE);

    response = client
        .post(OPENAI_BASE_URL)
        .timeout(Duration::from_secs(timeout))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(request)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    print_pre(&response, DEBUG_POST);

    if response.get("error") != None && retry > 0 {
        let mut n_count = 0;
        while n_count < retry {
            n_count += 1;
            println!(
                "Retry {}. Error: {:?}", 
                n_count, 
                response.get("error")
            );
            // Wait for 2 sec
            tokio::time::sleep(Duration::from_secs(2)).await;
            response = client
                .post(OPENAI_BASE_URL)
                .timeout(Duration::from_secs(timeout))
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(request)
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;
            
            if response.get("error") == None {
                break;
            }
        }
    }
    
    let response_string = response.to_string();
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