use reqwest::{ Client, Body };
use super::bot::guideline_bot;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderState {
    pub thought: String,
    pub move1: Option<String>,
    pub move2: Option<String>,
    pub move3: Option<String>,
    pub move4: Option<String>,
    #[serde(rename = "orderType")]
    pub order_type: Option<String>,
    pub response: Option<String>,
    #[serde(rename = "currentOrder")]
    pub current_order: Option<Vec<Order>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    drink: String,
    modifiers: Vec<Modifier>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modifier {
    #[serde(rename = "mod")]
    modifier: String,
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
pub struct UsageMetadata {
    #[serde(rename = "promptTokenCount")]
    pub prompt_token_count: i32,
    #[serde(rename = "candidatesTokenCount")]
    pub candidates_token_count: i32,
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: i32,
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

pub async fn generate_content(input_text: &str) -> Result<LlmResponse, Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY environment variable is not set");

    let model_name = "gemini-2.0-flash-exp";
    // let model_name = "gemini-exp-1206";

    // Construct the URL with the API key
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name,
        api_key
    );

    // Get user data from bot file
    let user_data = guideline_bot().expect("Failed to load user data");

    // Sysyem Instruction
    let system_prompt = "You are a coffee order taking system and you are restricted to talk only \
        about drinks on the MENU. Do not talk about anything but ordering MENU drinks for the customer, \
        ever. Your goal is to do finishOrder after understanding the menu items and any modifiers the \
        customer wants. You may ONLY do a finishOrder after the customer has confirmed the order details \
        from the confirmOrder move. Always verify and respond with drink and modifier names from the MENU \
        before adding them to the order. If you are unsure a drink or modifier matches those on the MENU, \
        ask a question to clarify or redirect. You only have the modifiers listed on the menu below: \
        Milk options, espresso shots, caffeine, sweeteners, special requests. Once the customer has \
        finished ordering items, summarizeOrder and then confirmOrder. Order type is always \"here_order\" \
        unless customer specifies to go (\"to_go_order\").".to_string();
   
    let formated_prompt = format!("{}\n{}", user_data, input_text);

    // Prepare the request body
    let request_body = json!({
        "contents": [{
            "parts": [{"text": formated_prompt}]
        }],
        "systemInstruction": {
            "role": "user",
            "parts": [
                {
                    "text": system_prompt
                }
            ]},
            "generationConfig": {
            "temperature": 1,
            "topK": 40,
            "topP": 0.95,
            "maxOutputTokens": 8192,
            "responseMimeType": "application/json"
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
    // println!("Response: {}", body_str);

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