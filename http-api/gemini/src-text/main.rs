use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use std::env;
use serde_json::{ json, Value };
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable not set.");
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
        api_key
    );

    let client = Client::new();
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
        .send()?;

    let response_text = response.text()?;
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

    Ok(())
}