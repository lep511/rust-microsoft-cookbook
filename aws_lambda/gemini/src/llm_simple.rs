use std::env;
use reqwest::header::CONTENT_TYPE;
use serde_json::{Value, json};
use std::error::Error as envError;

pub async fn invoke_simple_llm() -> Result<(), Box<dyn envError>> {
    let api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable not set.");
    let url = format!(
        //"https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-8b-exp-0924:generateContent?key={}",
        api_key
    );

    let client = reqwest::Client::new();
    let request_body = json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": "Explain how AI works"
                    }
                ]
            }
        ]
    });

    let response = client.post(&url)
        .header(CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;
    //println!("Response: {:?}", response_text);
    
    let response_json: Value = serde_json::from_str(&response_text)?;

    if let Some(text) = response_json["candidates"]
        .get(0)
        .and_then(|candidate| candidate["content"]["parts"].get(0))
        .and_then(|part| part["text"].as_str())
    {
        println!("Extracted Text: {}", text);
    } else {
        println!("Text not generated");
    }

    Ok(())
}