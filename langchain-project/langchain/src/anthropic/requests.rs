use reqwest::{Client, Response};
use crate::anthropic::libs::{
    ChatRequest, EmbedRequest, AnthropicEmbedEndpoint, ErrorResponse
};
use crate::anthropic::utils::print_pre;
use crate::anthropic::{
    ANTHROPIC_VERSION, ANTHROPIC_BASE_URL, DEBUG_PRE, DEBUG_POST, RETRY_BASE_DELAY,
    ANTHROPIC_EMBED_URL, ANTHROPIC_EMBEDMUL_URL, ANTHROPIC_EMBEDRANK_URL
};
use crate::llmerror::AnthropicError;
use std::time::Duration;
use serde_json::Value;
use tokio::time::sleep;

/// Sends a chat request to the Anthropic API with retry functionality.
///
/// # Arguments
///
/// * `request` - A reference to a `ChatRequest` struct containing the chat request details.
/// * `api_key` - A string slice containing the API key for authentication.
/// * `timeout` - The timeout duration for each request attempt in seconds.
/// * `max_retries` - The maximum number of retry attempts for failed requests.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(String)` containing the JSON response as a string if the request is successful.
/// - `Err(AnthropicError)` if there's an error during the request or response processing.
///
pub async fn request_chat(
    request: &ChatRequest,
    api_key: &str,
    timeout: u64,
    max_retries: u32,
) -> Result<String, AnthropicError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    print_pre(&request, DEBUG_PRE);
        
    // Serializes the request struct into a JSON byte vector
    let request_body = serde_json::to_vec(request)?;

    let mut response: Response = make_request(
        &client,
        api_key, 
        &request_body, 
        timeout,
    ).await?;

    for attempt in 1..=max_retries {
        if response.status().is_success() {
            break;
        }

        println!(
            "Retry {}/{}. Code error: {:?}", 
            attempt,
            max_retries,
            response.status()
        );

        sleep(RETRY_BASE_DELAY).await;
        
        response = make_request(
            &client,
            api_key,
            &request_body,
            timeout,
        ).await?;
    }

    // Checks if the response status is not successful (i.e., not in the 200-299 range).
    if !response.status().is_success() {
        println!("Response code: {}", response.status());
        match response.json::<ErrorResponse>().await {
            Ok(error_detail) => {
                return Err(AnthropicError::GenericError {
                    message: error_detail.error.message,
                    detail: "ERROR-req-9822".to_string(),
                });
            }
            Err(e) => {
                return Err(AnthropicError::GenericError {
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

/// Sends an embedding request to the Anthropic API using the specified endpoint.
///
/// # Arguments
///
/// * `request` - A reference to an `EmbedRequest` struct containing the request details.
/// * `api_key` - A string slice containing the API key for authentication.
/// * `endpoint` - An `AnthropicEmbedEndpoint` enum specifying which embedding endpoint to use.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(String)` containing the JSON response as a string if the request is successful.
/// - `Err(AnthropicError)` if there's an error during the request or response processing.
///
pub async fn request_embed(
    request: &EmbedRequest,
    api_key: &str,
    endpoint: AnthropicEmbedEndpoint
) -> Result<String, AnthropicError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    
    // Determines the appropriate URL based on the specified endpoint.
    let request_url = match endpoint {
        AnthropicEmbedEndpoint::Embed => ANTHROPIC_EMBED_URL,
        AnthropicEmbedEndpoint::MultimodalEmbed => ANTHROPIC_EMBEDMUL_URL,
        AnthropicEmbedEndpoint::Rerank => ANTHROPIC_EMBEDRANK_URL,
    };

    // Serializes the request struct into a JSON byte vector
    let request_body = serde_json::to_vec(request)?;
    
    print_pre(&request, DEBUG_PRE);

    let response: Response = make_embed_request(
        &client, 
        &request_url,
        api_key, 
        &request_body, 
    ).await?;

    // Checks if the response status is not successful (i.e., not in the 200-299 range).
    if !response.status().is_success() {
        println!("Response code: {}", response.status());
        
        return Err(AnthropicError::GenericError {
            message: "Error in request_embed".to_string(),
            detail: "ERROR-req-9835".to_string(),
        });
    }

    let response_data = response.json::<serde_json::Value>().await?;
    print_pre(&response_data, DEBUG_POST);

    let response_string = response_data.to_string();
    Ok(response_string)
}

/// Performs an HTTP GET request to the specified URL using the provided API key.
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL for the GET request.
/// * `api_key` - A string slice containing the API key for authentication.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(Value)` containing the JSON response if the request is successful.
/// - `Err(AnthropicError)` if there's an error during the request or response processing.
///
pub async fn get_request(
    url: &str, 
    api_key: &str
) -> Result<Value, AnthropicError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let response: Response = make_get_request(
        &client, 
        url, 
        api_key
    ).await?;

    if !response.status().is_success() {
        println!("Response code: {}", response.status());
        match response.json::<ErrorResponse>().await {
            Ok(error_detail) => {
                return Err(AnthropicError::GenericError {
                    message: error_detail.error.message,
                    detail: "ERROR-req-9822".to_string(),
                });
            }
            Err(e) => {
                return Err(AnthropicError::GenericError {
                    message: format!("Error: {}", e),
                    detail: "ERROR-req-9823".to_string(),
                });
            }
        }
    }
    
    let response_data = response.json::<Value>().await?;

    print_pre(&response_data, DEBUG_POST);

    Ok(response_data)
}

/// Makes an HTTP POST request to the Anthropic API endpoint
///
/// Sends a request with the specified parameters and handles authentication and headers
/// required by the Anthropic API.
///
/// # Arguments
///
/// * `client` - The HTTP client instance used to make the request
/// * `api_key` - The authentication API key for the Anthropic service
/// * `request_value` - The JSON payload to be sent in the request body
/// * `timeout` - The request timeout duration in seconds
///
/// # Returns
///
/// * `Result<Response, reqwest::Error>` - The HTTP response on success, or an error if the request fails
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
    api_key: &str,
    request_body: &[u8],
    timeout: u64,
) -> Result<Response, reqwest::Error> {
    Ok(client
        .post(ANTHROPIC_BASE_URL)
        .timeout(Duration::from_secs(timeout))
        .header("x-api-key", api_key)
        .header("anthropic-version", ANTHROPIC_VERSION)
        .header("Content-Type", "application/json")
        .body(request_body.to_vec())
        .send()
        .await?)
}

/// Makes an HTTP POST request to generate embeddings from the specified API endpoint
///
/// Sends a request with the necessary authentication and headers to generate
/// embeddings from the provided content.
///
/// # Arguments
///
/// * `client` - The HTTP client instance used to make the request
/// * `url` - The endpoint URL to send the embedding request to
/// * `api_key` - The authentication API key used in the Bearer token
/// * `request_value` - The JSON payload containing the content to be embedded
///
/// # Returns
///
/// * `Result<Response, reqwest::Error>` - The HTTP response on success, or an error if the request fails
///
/// # Errors
///
/// Returns a `reqwest::Error` if:
/// * The request fails to send
/// * There are network-related issues
/// * The server returns an error response
///
pub async fn make_embed_request(
    client: &Client,
    url: &str,
    api_key: &str,
    request_body: &[u8],
) -> Result<Response, reqwest::Error> {
    Ok(client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("anthropic-version", ANTHROPIC_VERSION)
        .header("Content-Type", "application/json")
        .body(request_body.to_vec())
        .send()
        .await?)
}

/// Makes an HTTP GET request to the specified API endpoint
///
/// Sends a GET request with the required authentication and headers
/// to retrieve data from the specified URL.
///
/// # Arguments
///
/// * `client` - The HTTP client instance used to make the request
/// * `url` - The endpoint URL to send the GET request to
/// * `api_key` - The authentication API key for the service
///
/// # Returns
///
/// * `Result<Response, reqwest::Error>` - The HTTP response on success, or an error if the request fails
///
/// # Errors
///
/// Returns a `reqwest::Error` if:
/// * The request fails to send
/// * There are network-related issues
/// * The server returns an error response
///
pub async fn make_get_request(
    client: &Client,
    url: &str,
    api_key: &str,
) -> Result<Response, reqwest::Error> {
    Ok(client
        .request(reqwest::Method::GET, url)
        .header("x-api-key", api_key)
        .header("anthropic-version", ANTHROPIC_VERSION)
        .header("Accept", "application/json")
        .send()
        .await?)
}