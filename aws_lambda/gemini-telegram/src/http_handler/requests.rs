use reqwest::{Client, Response};
use reqwest::{self, header::{HeaderMap, HeaderValue}};
use async_stream::stream;
use futures::StreamExt;
use super::libs::{ChatRequest, Part, Content, ChatResponse};
use super::libs::{CacheRequest, InlineData, EmbedRequest};
use super::utils::{print_pre, get_mime_type};
use super::{DEBUG_PRE, DEBUG_POST};
use base64::decode;
use serde_json::json;
use std::time::Duration;

// ======== REQUEST CHAT ===========
/// Makes an async HTTP POST request to chat endpoint with the provided chat request
///
/// # Arguments
///
/// * `url` - The endpoint URL to send the chat request to
/// * `request` - The chat request object containing the message payload
/// * `timeout` - Request timeout duration in seconds
/// * `retry` - Maximum number of retry attempts if the request fails
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
    retry: u32,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let mut response: serde_json::Value;
    
    print_pre(&request, DEBUG_PRE);

    response = client
        .post(url)
        .timeout(Duration::from_secs(timeout))
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
                .post(url.to_string())
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

// ======== UPLOAD MEDIA ===========
pub async fn upload_media(
    url: &str,
    base64_string: String,
    display_name: &str,
    content_length: &str,
    mut mime_type: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("X-Goog-Upload-Protocol", HeaderValue::from_static("resumable"));
    headers.insert("X-Goog-Upload-Command", HeaderValue::from_static("start"));
    headers.insert("X-Goog-Upload-Header-Content-Length", HeaderValue::from_str(content_length)?);
    headers.insert("X-Goog-Upload-Header-Content-Type", HeaderValue::from_str(mime_type)?);
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

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

    let body_data = decode(base64_string).expect("Invalid base64 string");

    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let upload_resp: serde_json::Value = client
        .post(upload_url)
        .headers(upload_headers)
        .body(body_data)
        .send()
        .await?
        .json()
        .await?;

    print_pre(&upload_resp, DEBUG_POST);
    
    // Wait for video processing
    if mime_type.starts_with("video") {
        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    let file_uri = upload_resp["file"]["uri"]
        .as_str()
        .ok_or("Missing file URI")?
        .trim_matches('"')
        .to_string();

    Ok(file_uri)
}

// ======== REQUEST CACHE ===========
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
/// * `retry` - Maximum number of retry attempts if the request fails
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
    ttl: u32
) -> Result<String, Box<dyn std::error::Error>> {

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

    let cache_request = CacheRequest {
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

    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    
    let cache_resp: serde_json::Value = client
        .post(url)
        .headers(headers)
        .json(&cache_request)
        .send()
        .await?
        .json()
        .await?;

    print_pre(&cache_resp, DEBUG_POST);

    let cache_name = cache_resp["name"]
        .as_str()
        .ok_or("Missing cache name")?
        .trim_matches('"')
        .to_string();

    Ok(cache_name)
}

// ======== REQUEST EMBED ===========
/// Sends an embedding request to generate vector embeddings for input text
///
/// # Arguments
///
/// * `url` - The endpoint URL for the embedding service
/// * `request` - The embedding request containing the input text and model parameters
/// * `retry` - Maximum number of retry attempts if the request fails
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
    retry: u32,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let mut response: serde_json::Value;
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
 
    print_pre(&request, DEBUG_PRE);
  
    response = client
        .post(url.to_string())
        .header("Content-Type", "application/json")
        .json(&request)
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
                .post(url.to_string())
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