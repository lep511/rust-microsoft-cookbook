use reqwest::{ Client, Body };
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedContents {
    #[serde(rename = "cachedContents")]
    cached_contents: Option<Vec<CacheResponse>>,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmResponse {
    pub gemini_response: GeminiResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: UsageMetadata,
    #[serde(rename = "modelVersion")]
    pub model_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub content: Content,
    #[serde(rename = "finishReason")]
    pub finish_reason: String,
    pub index: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafetyRating {
    pub category: String,
    pub probability: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

fn read_and_encode(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Read the file contents
    let contents = fs::read(file_path)?;
    
    // Convert to base64
    let base64_string = BASE64.encode(contents);
    
    Ok(base64_string)
}

fn get_matching_name(response: &CachedContents) -> Option<String> {
    if let Some(contents) = &response.cached_contents {
        for content in contents {
            if content.display_name == "Transcript Analysis" {
                return Some(content.name.clone());
            }
        }
    }
    None
}

async fn cached_contents() -> Result<String, Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY environment variable is not set");

    // Create a reqwest client
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    // Construct the URL with the API key for check cache list
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/cachedContents?key={}",
        api_key
    ); 
    
    let response = client
        .get(url)
        .send()
        .await?;

    if response.status().is_success() {
        let body_str = response.text().await?;
        let response: CachedContents = serde_json::from_str(&body_str).unwrap();
        let cache_name = get_matching_name(&response);
        match cache_name {
            Some(cache_name) => {
                println!("Found matching content name: {}", cache_name);
                return Ok(cache_name);
            },
            None => {
                println!("No cache found");
            }
        }

    } else {
        println!("Error list cache: {}", response.status());
    }
    
    let gemini_model = "models/gemini-1.5-flash-001".to_string();

    // Construct the URL with the API key for save cache
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/cachedContents?key={}",
        api_key
    );

    let cache_file = "a11.txt";

    let file_content_b64 = match read_and_encode(cache_file) {
        Ok(encoded) => encoded,
        Err(e) => {
            println!("[ERROR] Error convert to base64: {}", e);
            String::from("No data.")
        }
    };

    // Prepare the request body
    let request_body = json!({
        "model": gemini_model,
        "contents": [
            {
                "parts": [
                    {
                        "inline_data": {
                            "mime_type": "text/plain",
                            "data": file_content_b64
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
        "ttl": "600s",
        "displayName": "Transcript Analysis"
    });

    let request_body = serde_json::to_string(&request_body)?;
    let body: Body = Body::wrap(request_body);

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
    let cache_name = cache_response.name.clone();
    
    Ok(cache_name)
}

pub async fn get_gemini_response(prompt: &str) -> Result<LlmResponse, Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY environment variable is not set");

    // Construct the URL with the API key
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-001:generateContent?key={}",
        api_key
    );

    let cache_name: String = cached_contents().await?;
    println!("[INFO] Gemini cache name: {}", cache_name);

    // Prepare the request body
    let request_body = json!({
        "contents": [{
            "parts": [{"text": prompt}],
            "role": "user"
        }],
        "cachedContent": cache_name
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

    match serde_json::from_str::<GeminiResponse>(&body_str) {
        Ok(response_data) => {
            println!("Model Version: {}", response_data.model_version);
            println!("Total Token Count: {}", response_data.usage_metadata.total_token_count);
            let response_llm: LlmResponse = LlmResponse {
                gemini_response: response_data,
            };
            Ok(response_llm)
        }
        Err(e) => Err(Box::new(e)),
    }
}