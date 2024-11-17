use reqwest::{ Client, Body };
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheResponse {
    pub name: String,
    pub model: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    #[serde(rename = "expireTime")]
    pub expire_time: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: UsageMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageMetadata {
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: i64,
}

fn read_and_encode(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Read the file contents
    let contents = fs::read(file_path)?;
    
    // Convert to base64
    let base64_string = BASE64.encode(contents);
    
    Ok(base64_string)
}

async fn cached_contents(base64_data: &str) -> Result<CacheResponse, Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY environment variable is not set");

    let gemini_model = "models/gemini-1.5-flash-001".to_string();

    // Construct the URL with the API key
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/cachedContents?key={}",
        api_key
    );

    // Prepare the request body
    let request_body = json!({
        "model": gemini_model,
        "contents": [
            {
                "parts": [
                    {
                        "inline_data": {
                            "mime_type": "text/plain",
                            "data": base64_data
                        }
                    }
                ],
                "role": "user"
            }
        ],
        "system_instruction": {
            "parts": [
                {
                    "text": "You are an expert at analyzing transcripts."
                }
            ]
        },
        "ttl": "600s"
    });

    let request_body = serde_json::to_string(&request_body)?;
    let body: Body = Body::wrap(request_body);

    // Create a reqwest client
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;  

    // Send the POST request
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    // Print status code
    println!("Status: {}", response.status());

    // Print headers if needed
    // println!("Headers: {:#?}", response.headers());

    if response.status().as_u16() > 299 {
        println!("Error: {}", response.status());
        return Err("Error in Gemini API".into());
    }

    // Read the response body
    let body_str: String = response.text().await?;

    // Parse and print the response
    println!("Response: {}", body_str);
    
    let cache_response: CacheResponse = serde_json::from_str(&body_str)?;
    
    Ok(cache_response)
}

pub async fn get_gemini_response(file_cache_path: &str) {
    let file_content_b64 = match read_and_encode(file_cache_path) {
        Ok(encoded) => encoded,
        Err(e) => {
            println!("[ERROR] Error convert to base64: {}", e);
            String::from("No data.")
        }
    };

    let cache_response: CacheResponse = match cached_contents(&file_content_b64).await {
        Ok(response) => response,
        Err(e) => {
            println!("[ERROR] Error in Gemini API: {}", e);
            return;
        }
    };
    println!("[INFO] Gemini API Response: {:?}", cache_response);
}