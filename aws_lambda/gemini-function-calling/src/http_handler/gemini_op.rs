use reqwest::{ Client, Body };
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::fs::File;

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
pub struct Part {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

pub async fn get_gemini_response(prompt: &str) -> Result<LlmResponse, Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY environment variable is not set");

    // Construct the URL with the API key
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro-latest:generateContent?key={}",
        api_key
    );

    // Construct request body
    let tools_str = std::fs::read_to_string("file_tools.json")?;

    // Prepare the request body
    let request_body = json!({
        "system_instruction": {
            "parts": {"text": "You are a helpful lighting system bot. You can turn lights on and off, and you can set the color. Do not perform any other tasks."}
        },
        "tools": [tools_str],
        "tool_config": {
            "function_calling_config": {"mode": "none"}
        },
        "contents": {
            "role": "user",
            "parts": {
                "text": "What can you do?"
            }
        }
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