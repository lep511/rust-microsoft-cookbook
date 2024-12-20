use reqwest::{ Client, Body };
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::File;

pub async fn get_gemini_response(
    prompt: &str, 
    google_api_token: String
    ) -> Result<String, Box<dyn std::error::Error>> {
    
    println!("Api token: {}", google_api_token);
    // Construct the URL with the API key
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
        google_api_token
    );

    // Prepare the request body
    let request_body = json!({
        "contents":[
            {
                "role":"user",
                "parts":[
                    {
                        "text": prompt
                    }
                ]
            }
        ],
        "generationConfig":{
            "temperature":1,
            "topK":40,
            "topP":0.95,
            "maxOutputTokens":8192,
            "response_mime_type": "application/json",
            "response_schema": {
            "type": "ARRAY",
            "items": {
                "type": "OBJECT",
                "properties": {
                    "recipe_name": {"type":"STRING"},
                }
            }
        }
    }});

    // Create a reqwest client
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;  

    // Send the POST request
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
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

    let result: Value = response.json().await?;
    println!("{:#?}", result);

    // Read the response body
    // let body_str: String = response.text().await?;
    let body_str: String = String::from("test");

    Ok(body_str)
}