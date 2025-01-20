use reqwest::Client;
use crate::anthropic::libs::ChatRequest;
use crate::anthropic::utils::print_pre;
use crate::anthropic::{
    ANTHROPIC_VERSION, ANTHROPIC_BASE_URL, DEBUG_PRE, DEBUG_POST
};
use crate::llmerror::AnthropicError;
use std::time::Duration;

pub async fn request_chat(
    request: &ChatRequest,
    api_key: &str,
    timeout: u64,
    retry: u32,
) -> Result<String, AnthropicError> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let mut response: serde_json::Value;
    
    print_pre(&request, DEBUG_PRE);

    response = client
        .post(ANTHROPIC_BASE_URL)
        .timeout(Duration::from_secs(timeout))
        .header("x-api-key", api_key)
        .header("anthropic-version", ANTHROPIC_VERSION)
        .header("Content-Type", "application/json")
        .json(request)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    print_pre(&response, DEBUG_POST);

    if response.get("error") != None && retry > 0 {
        let mut n_count: u32 = 0;
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
                .post(ANTHROPIC_BASE_URL)
                .timeout(Duration::from_secs(timeout))
                .header("anthropic-version", ANTHROPIC_VERSION)
                .header("Content-Type", "application/json")
                .json(&request)
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