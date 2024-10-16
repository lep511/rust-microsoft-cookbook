use reqwest::Client;
use serde_json::{Value, json};
use std::env;

pub async fn invoke_json_llm() -> Result<(), Box<dyn std::error::Error>> {
    let google_api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable not set.");
    //let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", google_api_key);
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-8b-exp-0924:generateContent?key={}", google_api_key);

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
        .send()
        .await?;

    let response_text = response.text().await?;
    
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