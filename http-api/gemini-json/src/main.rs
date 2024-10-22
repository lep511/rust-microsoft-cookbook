use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use std::env;
use serde::{Serialize, Deserialize};
use serde_json::{ json, Value };
use std::error::Error;
use anyhow::anyhow;

#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    recipe_name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("GEMINI_API_KEY")
        .map_err(|_| anyhow!("GEMINI_API_KEY environment variable not set."))?;
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
        api_key
    );

    let client = Client::new();
    let response = client.post(&url)
        .header("Content-Type", "application/json")
        .body(r#"
            {
                "contents":[
                    {
                        "parts":[
                            {
                                "text":"List 5 popular cookie recipes"
                            }
                        ]
                    }
                ],
                "generationConfig":{
                    "response_mime_type":"application/json",
                    "response_schema":{
                        "type":"ARRAY",
                        "items":{
                            "type":"OBJECT",
                            "properties":{
                                "recipe_name":{
                                    "type":"STRING"
                                }
                            }
                        }
                    }
                }
            }
        "#)
        .send()?;

    let response_text = response.text()?;

    let response_json: Value = serde_json::from_str(&response_text)?;
    
    if let Some(text) = response_json["candidates"]
        .get(0)
        .and_then(|candidate| candidate["content"]["parts"].get(0))
        .and_then(|part| part["text"].as_str())
    {
        println!("Response json: {}", text);
    } else {
        println!("Text not generated");
    }

    Ok(())
}