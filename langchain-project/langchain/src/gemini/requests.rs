use reqwest::{Client, Response};
use reqwest::{self, header::{HeaderMap, HeaderValue}};
use log::{info, warn, error};
use async_stream::stream;
use futures::StreamExt;
use crate::gemini::libs::{ChatRequest, Part, Content, ChatResponse};
use crate::gemini::libs::{CacheRequest, InlineData, EmbedRequest};
use crate::gemini::utils::print_pre;
use crate::gemini::{DEBUG_PRE, DEBUG_POST, RETRY_BASE_DELAY};
use crate::llmerror::GeminiError;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Request Chat ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Makes an async HTTP POST request to chat endpoint with the provided chat request
///
/// # Arguments
///
/// * `url` - The endpoint URL to send the chat request to
/// * `request` - The chat request object containing the message payload
/// * `timeout` - Request timeout duration in seconds
/// * `max_retries` - The maximum number of retry attempts for failed requests.
///
/// # Returns
///
/// * `Result<String, Box<dyn std::error::Error>>` - Returns the response body as a String on success,
///   or a boxed error on failure
///
/// # Errors
///
/// This function will return an error if:
/// * The HTTP client cannot be built
/// * The request fails to send
/// * The response cannot be parsed as JSON
/// * The request times out
pub async fn request_chat(
    url: &str, 
    request: &ChatRequest, 
    timeout: u64,
    max_retries: u32,
) -> Result<String, GeminiError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    // Serializes the request struct into a JSON byte vector
    let request_body = serde_json::to_vec(request)?;

    let mut response: Response = make_request(
        &client,
        url, 
        &request_body, 
        timeout,
    ).await?;
    
    print_pre(&request, DEBUG_PRE);

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
            url,
            &request_body,
            timeout,
        ).await?;
    }

    // Checks if the response status is not successful (i.e., not in the 200-299 range).
    if !response.status().is_success() {
        let gemini_error: GeminiError = manage_error(response).await;
        return Err(gemini_error);
    }

    let response_data = response.json::<serde_json::Value>().await?;
    print_pre(&response_data, DEBUG_POST);
    
    let response_string = response_data.to_string();
    Ok(response_string)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Upload Media ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub async fn upload_media(
    url: &str,
    base64_string: String,
    display_name: &str,
    content_length: &str,
    mime_type: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let mut headers = HeaderMap::new();
    headers.insert("X-Goog-Upload-Protocol", HeaderValue::from_static("resumable"));
    headers.insert("X-Goog-Upload-Command", HeaderValue::from_static("start"));
    headers.insert("X-Goog-Upload-Header-Content-Length", HeaderValue::from_str(content_length)?);
    headers.insert("X-Goog-Upload-Header-Content-Type", HeaderValue::from_str(mime_type)?);
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let initial_resp = client
        .post(url)
        .headers(headers)
        .json(&json!({
            "file": {
                "display_name": display_name
            }
        }))
        .send()
        .await?;

    // Get upload URL from response headers
    let upload_url = initial_resp
        .headers()
        .get("x-goog-upload-url")
        .ok_or("Missing upload URL")?
        .to_str()?;

    // Upload file content
    let mut upload_headers = HeaderMap::new();
    upload_headers.insert("Content-Length", HeaderValue::from_str(content_length)?);
    upload_headers.insert("X-Goog-Upload-Offset", HeaderValue::from_static("0"));
    upload_headers.insert("X-Goog-Upload-Command", HeaderValue::from_static("upload, finalize")); 

    let body_data = STANDARD.decode(base64_string).expect("Invalid base64 string");

    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let upload_resp = client
        .post(upload_url)
        .headers(upload_headers)
        .body(body_data)
        .send()
        .await?;

    // Checks if the response status is not successful (i.e., not in the 200-299 range).
    if !upload_resp.status().is_success() {
        let gemini_error: GeminiError = manage_error(upload_resp).await;
        return Err(Box::new(gemini_error));
    }

    let response_data = upload_resp.json::<serde_json::Value>().await?;
    print_pre(&response_data, DEBUG_POST);
    
    // Wait for video processing
    if mime_type.starts_with("video") {
        sleep(Duration::from_secs(5)).await;
    }

    let file_uri = response_data["file"]["uri"]
        .as_str()
        .ok_or("Missing file URI")?
        .trim_matches('"')
        .to_string();

    Ok(file_uri)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Request Cache ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Submits data to a caching service with model-specific instructions and TTL
///
/// # Arguments
///
/// * `url` - The endpoint URL for the caching service
/// * `data` - The data to be cached
/// * `mime_type` - MIME type of the data being cached
/// * `instruction` - System instruction for processing the data
/// * `model` - The AI model identifier to be used
/// * `ttl` - Time-to-live duration in seconds for the cached data
///
/// # Returns
///
/// * `Result<String, Box<dyn std::error::Error>>` - Returns the cache entry name/identifier on success,
///   or a boxed error on failure
///
/// # Details
///
/// This function creates a cache request with:
/// * Inline data with specified MIME type
/// * System instruction as user content
/// * Model specification with formatted name
/// * TTL duration in seconds
///
/// # Errors
///
/// This function will return an error if:
/// * The HTTP client cannot be built
/// * The cache request fails to send
/// * The response cannot be parsed as JSON
/// * The cache name is missing from the response
pub async fn request_cache(
    url: String,
    data: String,
    mime_type: String,
    instruction: String,
    model: &str,
    ttl: u32,
) -> Result<String, GeminiError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let model_name = format!("models/{}", model);

    let part_system_instruction = vec![Part {
        text: Some(instruction),
        function_call: None,
        function_response: None,
        inline_data: None,
        file_data: None,
    }];

    let system_instruction = Content {
        role: "user".to_string(),
        parts: part_system_instruction,
    };

    let ttl = format!("{}s", ttl);

    let request = CacheRequest {
        model: model_name,
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: None,
                function_call: None,
                function_response: None,
                inline_data: Some(InlineData {
                    mime_type: mime_type.to_string(),
                    data: Some(data),
                }),
                file_data: None,
            }],
        }],
        system_instruction: system_instruction,
        ttl: ttl,
    };

    // Serializes the request struct into a JSON byte vector
    let request_body = serde_json::to_vec(&request)?;
    let timeout = 2400;

    let response: Response = make_request(
        &client,
        &url, 
        &request_body, 
        timeout,
    ).await?;

    // Checks if the response status is not successful (i.e., not in the 200-299 range).
    if !response.status().is_success() {
        let gemini_error: GeminiError = manage_error(response).await;
        return Err(gemini_error);
    }

    let response_data = response.json::<serde_json::Value>().await?;
    print_pre(&response_data, DEBUG_POST);
    
    let cache_name = response_data["name"]
        .as_str()
        .ok_or_else(|| GeminiError::GenericError { 
            message: "Missing cache name".to_string(),
            detail: "ERROR-req-9877".to_string(),
        })?
        .trim_matches('"')
        .to_string();

    Ok(cache_name)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Request Embed ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Sends an embedding request to generate vector embeddings for input text
///
/// # Arguments
///
/// * `url` - The endpoint URL for the embedding service
/// * `request` - The embedding request containing the input text and model parameters
/// * `max_retries` - Maximum number of retry attempts if the request fails
///
/// # Returns
///
/// * `Result<String, Box<dyn std::error::Error>>` - Returns the embedding response as a JSON string on success,
///   or a boxed error on failure
/// 
/// # Details
///
/// This function:
/// * Creates an HTTPS client with TLS support
/// * Prints the request details before sending
/// * Makes a POST request with JSON payload
/// * Prints the response details
/// * Returns the response as a string
///
/// # Errors
///
/// This function will return an error if:
/// * The HTTP client cannot be built
/// * The request fails to send
/// * The response cannot be parsed as JSON
pub async fn request_embed(
    url: &str,
    request: EmbedRequest,
    max_retries: u32,
) -> Result<String, GeminiError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
 
    print_pre(&request, DEBUG_PRE);

    let timeout = 2400;
    // Serializes the request struct into a JSON byte vector
    let request_body = serde_json::to_vec(&request)?;
    
    let mut response: Response = make_request(
        &client,
        url, 
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
            url, 
            &request_body, 
            timeout,
        ).await?;
    }

    // Checks if the response status is not successful (i.e., not in the 200-299 range).
    if !response.status().is_success() {
        let gemini_error: GeminiError = manage_error(response).await;
        return Err(gemini_error);
    }

    let response_data = response.json::<serde_json::Value>().await?;
    print_pre(&response_data, DEBUG_POST);
    
    let response_string = response_data.to_string();
    Ok(response_string)
}

pub fn strem_chat(
    url: String,
    request: ChatRequest,
) -> impl futures::Stream<Item = ChatResponse> {
    stream! {
        let client = Client::new();

        let response: Response = match client
            .post(url)
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

/// Makes an HTTP POST request to the Anthropic API endpoint
///
/// Sends a request with the specified parameters and handles authentication and headers
/// required by the Anthropic API.
///
/// # Arguments
///
/// * `client` - The HTTP client instance used to make the request
/// * `url` - The endpoint URL to send the POST request to
/// * `request_body` - Byte slice containing the JSON request body
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
pub async fn make_request(
    client: &Client,
    url: &str,
    request_body: &[u8],
    timeout: u64,
) -> Result<Response, reqwest::Error> {
    Ok(client
        .post(url)
        .timeout(Duration::from_secs(timeout))
        .header("Content-Type", "application/json")
        .body(request_body.to_vec())
        .send()
        .await?)
}

pub async fn manage_error(
    response: Response,
) -> GeminiError {
    error!("Response code: {}", response.status());

    match response.json::<ChatResponse>().await {
        Ok(error_detail) => {
            if let Some(error_message) = error_detail.error {
                if let Some(message) = error_message.message {
                    return GeminiError::GenericError {
                        message,
                        detail: "ERROR-req-9821".to_string(),
                    }
                };
            }
            GeminiError::GenericError {
                message: "Unknown error".to_string(),
                detail: "ERROR-req-9822".to_string(),
            }
        },
        Err(e) => {
            GeminiError::GenericError {
                message: format!("Error: {}", e),
                detail: "ERROR-req-9823".to_string(),
            }
        }
    }
}