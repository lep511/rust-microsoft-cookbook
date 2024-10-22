use anyhow::anyhow;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;
use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("GEMINI_API_KEY")
        .map_err(|_| anyhow!("GEMINI_API_KEY environment variable not set."))?;
    //let files = ["image_blog_post_creator1.jpeg"];
    let files = ["pizza.jpeg"];
    let client = Client::new();

    for file_name in files.iter() {
        let file_bytes = fs::read(file_name)?;
        let num_bytes = file_bytes.len();

        // Upload file
        let upload_url = format!("https://generativelanguage.googleapis.com/upload/v1beta/files?key={}", api_key);
        let mut headers = HeaderMap::new();
        headers.insert("X-Goog-Upload-Command", HeaderValue::from_static("start, upload, finalize"));
        headers.insert("X-Goog-Upload-Header-Content-Length", HeaderValue::from(num_bytes as u64));
        headers.insert("X-Goog-Upload-Header-Content-Type", HeaderValue::from_static("image/jpeg"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/jpeg"));

        let upload_response: Response = client.post(&upload_url)
            .headers(headers)
            .body(file_bytes)
            .send()?;

        let upload_response_text = upload_response.text()?;
        println!("Upload Response: {}", upload_response_text);

        // Parse the response to get the file URI
        let json_response: Value = serde_json::from_str(&upload_response_text)?;
        let file_uri = json_response["file"]["uri"]
            .as_str()
            .ok_or("Failed to extract file URI")?;

        // Generate content
        // let generation_url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key);
        // let generation_url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro-002:generateContent?key={}", api_key);
        let generation_url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-8b-exp-0924:generateContent?key={}", api_key);
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
                            "text": "Write a short, engaging blog post based on this picture. It should include a description of the meal in the photo and talk about my journey meal prepping."
                        }
                    ]
                },
                {
                    "role": "user",
                    "parts": [
                        {
                            "text": "INSERT_INPUT_HERE"
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

        let response = client.post(&generation_url)
            .header(CONTENT_TYPE, "application/json")
            .json(&generation_body)
            .send()?;

        let response_text = response.text()?;
        //println!("Generation Response: {}", response_text);
        let json: Value = serde_json::from_str(&response_text)?;

        if let Some(text) = json["candidates"]
            .get(0)
            .and_then(|candidate| candidate["content"]["parts"].get(0))
            .and_then(|part| part["text"].as_str())
        {
            println!("Extracted Text: {}", text);
        } else {
            println!("Text not generated");
        }
    }

    Ok(())
}