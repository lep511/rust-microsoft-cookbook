use reqwest::{Client, Response};
use log::{warn, error};
use crate::assembly::libs::{TranscriptRequest};
use crate::assembly::utils::print_pre;
use crate::assembly::{
    DEBUG_PRE, DEBUG_POST, RETRY_BASE_DELAY,
};
use crate::assembly::error::AssemblyError;
use std::time::Duration;
use std::collections::HashMap;
use tokio::time::sleep;

pub async fn request_engine(
    request: &TranscriptRequest,
    url: &str,
    api_key: &str,
    timeout: Duration,
    max_retries: u32,
) -> Result<String, AssemblyError> {
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
        if response.status().is_success() {
            break;
        }

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
        let anthropic_error: AssemblyError = manage_error(response).await;
        return Err(anthropic_error);   
    }

    let response_data = response.json::<serde_json::Value>().await?;
    print_pre(&response_data, DEBUG_POST);
    
    let response_string = response_data.to_string();
    Ok(response_string)
}

pub async fn upload_media(
    url: &str,
    api_key: &str,
    body: &[u8],
    timeout: Duration,
) -> Result<String, AssemblyError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let response = make_request(
        &client,
        url,
        api_key,
        body,
        timeout,
    ).await?;

    let response_data = response.json::<serde_json::Value>().await?;
    let response_string = response_data.to_string();
    let mut response_map: HashMap<String, String> = match serde_json::from_str(&response_string) {
        Ok(response_form) => response_form,
        Err(e) => {
            error!("Error UploadFileResponse: {:?}", e);
            return Err(AssemblyError::ResponseContentError);
        }
    };

    let upload_url = response_map.remove("upload_url").unwrap();

    Ok(upload_url)
}

pub async fn get_engine(
    url: &str,
    api_key: &str,
) -> Result<String, AssemblyError> {
    // Creates an HTTPS-capable client using rustls TLS implementation.
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let response = make_get_request(
        &client,
        url,
        api_key,
    ).await?;

    let response_data = response.json::<serde_json::Value>().await?;
    print_pre(&response_data, DEBUG_POST);

    let response_string = response_data.to_string();
    Ok(response_string)
}

/// Makes an HTTP POST request to the Assembly API endpoint
///
/// Sends a request with the specified parameters and handles authentication and headers
/// required by the Assembly API.
///
/// # Arguments
///
/// * `client` - The HTTP client instance used to make the request
/// * `url` - A string slice that holds the URL for the POST request.
/// * `api_key` - The authentication API key for the Assembly service
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
    url: &str,
    api_key: &str,
    request_body: &[u8],
    timeout: Duration,
) -> Result<Response, reqwest::Error> {
    Ok(client
        .post(url)
        .timeout(timeout)
        .header("Authorization", api_key)
        .header("Content-Type", "application/json")
        .body(request_body.to_vec())
        .send()
        .await?)
}

pub async fn make_get_request(
    client: &Client,
    url: &str,
    api_key: &str,
) -> Result<Response, reqwest::Error> {
    Ok(client
        .request(reqwest::Method::GET, url)
        .header("Authorization", api_key)
        .header("Content-Type", "application/json")
        .send()
        .await?)
}

pub async fn manage_error(
    response: Response,
) -> AssemblyError {
    error!("Response code: {}", response.status());

    match response.status().as_u16() {
        400 => {
            return AssemblyError::BadRequest;
        }
        401 => {
            return AssemblyError::Unauthorized;
        }
        429 => {
            return AssemblyError::TooManyRequest;
        }
        500 | 503 | 504 => {
            return AssemblyError::InternalServerError;
        }
        _ => {
            return AssemblyError::GenericError {
                message: "Unknown error.".to_string(),
                detail: "ERROR-req-9820".to_string(),
            };
        }
    }
}