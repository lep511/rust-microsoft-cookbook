use reqwest::{Client, Response};
use log::{warn, error};

// use crate::llm::{
//     DATADOG_LLM_URL_V1,
// };

// use crate::lib::{
//     ChatRequest, EmbedRequest, ErrorResponse, ChatResponse,
// };

use crate::error::DatadogError;
use crate::utils::print_pre;
use std::time::Duration;
use tokio::time::sleep;

pub async fn request_trace(
    request: &ChatRequest,
    api_key: &str,
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

pub async fn make_request(
    client: &Client,
    url: &str,
    api_key: &str,
    request_body: &[u8],
) -> Result<Response, reqwest::Error> {
    Ok(client
        .post(url)
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
                "permission_error" => OpenAIError::PermissionDeniedError(
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