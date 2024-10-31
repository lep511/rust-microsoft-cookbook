use anyhow::{anyhow, Error as AnyhowError};
use futures::future::join_all;
use reqwest::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;
use std::env;
use tokio::fs;
use tokio::time::{timeout, Duration};
use std::sync::Arc;
use reqwest::ClientBuilder;

// Configuration struct to hold common parameters
#[derive(Clone)]
struct Config {
    api_key: String,
    client: Arc<Client>,
    timeout_duration: Duration,
}

// Structure to hold file processing results
#[derive(Debug)]
struct ProcessingResult {
    file_name: String,
    text: String,
}

async fn read_file_buffered(file_name: &str) -> Result<Vec<u8>, AnyhowError> {
    fs::read(file_name).await.map_err(AnyhowError::from)
}

async fn upload_file(
    config: &Config,
    file_name: &str,
) -> Result<String, AnyhowError> {
    // Read file contents with buffered I/O
    let file_bytes = read_file_buffered(file_name).await?;
    let num_bytes = file_bytes.len();

    let upload_url = format!(
        "https://generativelanguage.googleapis.com/upload/v1beta/files?key={}",
        config.api_key
    );
    
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-Goog-Upload-Command",
        HeaderValue::from_static("start, upload, finalize"),
    );
    headers.insert(
        "X-Goog-Upload-Header-Content-Length",
        HeaderValue::from(num_bytes as u64),
    );
    headers.insert(
        "X-Goog-Upload-Header-Content-Type",
        HeaderValue::from_static("image/jpeg"),
    );
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("image/jpeg"),
    );

    // Send upload request with timeout
    let upload_response: Response = timeout(
        config.timeout_duration,
        config.client
            .post(&upload_url)
            .headers(headers)
            .body(file_bytes)
            .send()
    ).await??;

    let upload_response_text = upload_response.text().await?;

    // Parse response to get file URI
    let json_response: Value = serde_json::from_str(&upload_response_text)?;
    let file_uri = json_response["file"]["uri"]
        .as_str()
        .ok_or_else(|| anyhow!("Failed to extract file URI"))?;

    Ok(file_uri.to_string())
}

async fn generate_content(
    config: &Config,
    file_uri: &str,
    input_text: &str,
) -> Result<String, AnyhowError> {
    let generation_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-8b-exp-0924:generateContent?key={}",
        config.api_key
    );

    let generation_body = serde_json::json!({
        "contents": [
            {
                "role": "user",
                "parts": [
                    {
                        "fileData": {
                            "fileUri": file_uri,
                            "mimeType": "image/jpeg"
                        }
                    },
                    {
                        "text": input_text
                    }
                ]
            }
        ],
        "generationConfig": {
            "temperature": 0.9,
            "topK": 40,
            "topP": 0.95,
            "maxOutputTokens": 1024,
            "responseMimeType": "text/plain"
        }
    });

    // Send generation request with timeout
    let response = timeout(
        config.timeout_duration,
        config.client
            .post(&generation_url)
            .header(CONTENT_TYPE, "application/json")
            .json(&generation_body)
            .send()
    ).await??;

    let response_text = response.text().await?;
    let json: Value = serde_json::from_str(&response_text)?;

    let text = json["candidates"]
        .get(0)
        .and_then(|candidate| candidate["content"]["parts"].get(0))
        .and_then(|part| part["text"].as_str())
        .ok_or_else(|| anyhow!("Text not generated"))?;

    Ok(text.to_string())
}

async fn process_file(
    config: &Config,
    file_name: String,
) -> Result<ProcessingResult, AnyhowError> {
    // Upload file and get URI
    let file_uri = upload_file(config, &file_name).await?;
    
    let prompt = "Accurately identify the baked good in the image and provide an appropriate recipe consistent with your analysis.";
    
    // Generate content
    let text = generate_content(config, &file_uri, &prompt).await?;
    
    Ok(ProcessingResult {
        file_name,
        text,
    })
}

#[tokio::main]
async fn main() -> Result<(), AnyhowError> {
    // Get API key from environment
    let api_key = env::var("GEMINI_API_KEY")
        .map_err(|_| anyhow!("GEMINI_API_KEY environment variable not set."))?;

    // Create an optimized HTTP client with connection pooling
    let client = ClientBuilder::new()
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .timeout(Duration::from_secs(30))
        .build()?;

    let config = Config {
        api_key,
        client: Arc::new(client),
        timeout_duration: Duration::from_secs(30),
    };

    let files = vec![
        "pizza1.jpg".to_string(),
        "pizza2.jpg".to_string(),
        "pizza3.jpg".to_string(),
    ];

    // Process all files concurrently
    let tasks: Vec<_> = files
        .into_iter()
        .map(|file_name| {
            let config = config.clone();
            tokio::spawn(async move {
                process_file(&config, file_name).await
            })
        })
        .collect();

    // Wait for all tasks to complete and collect results
    let results = join_all(tasks).await;

    // Process results
    for result in results {
        match result {
            Ok(Ok(processing_result)) => {
                println!("File: {}", processing_result.file_name);
                println!("Generated Text: {}", processing_result.text);
                println!("---");
            },
            Ok(Err(e)) => println!("Error processing file: {}", e),
            Err(e) => println!("Task error: {}", e),
        }
    }

    Ok(())
}
