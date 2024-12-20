use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum GeminiChatError {
    #[error("Gemini API key not found in environment variables")]
    ApiKeyNotFound,
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    #[error("Failed to get response content")]
    ResponseContentError,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub contents: Vec<Content>,
    pub tools: Option<Vec<Value>>,
    #[serde(rename = "generationConfig")]
    pub generation_config: Option<GenerationConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(rename = "functionCall", default)]
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  GenerationConfig {
    pub temperature: f32,
    #[serde(rename = "topK")]
    pub top_p: f32,
    #[serde(rename = "topP")]
    pub top_k: i32,
    #[serde(rename = "maxOutputTokens")]
    pub max_output_tokens: i32,
    #[serde(rename = "responseMimeType")]
    pub response_mime_type: String,
}

#[derive(Debug, Clone)]
pub struct ChatGemini {
    pub base_url: String,
    pub client: Client,
}

impl ChatGemini {
    pub fn new(model: &str) -> Result<Self, GeminiChatError> {
        let api_key = match env::var("GEMINI_API_KEY") {
            Ok(key) => key,
            Err(env::VarError::NotPresent) => {
                println!("[ERROR] GEMINI_API_KEY not found in environment variables");
                return Err(GeminiChatError::ApiKeyNotFound);
            }
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(GeminiChatError::EnvError(e));
            }
        };
        
        let base_url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model,
            api_key,
        );
        
        Ok(Self {
            base_url,
            client: Client::builder()
                .use_rustls_tls()
                .build()?,
        })
    }

    pub async fn invoke(&self, contents: ChatRequest) -> Result<Response, GeminiChatError> {
        let response = self
            .client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .json(&contents)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        if response["error"].is_object() {
            println!("Response: {:?}", response);
            return Err(GeminiChatError::ResponseContentError)
        };

        let response = response.to_string();
        match serde_json::from_str(&response) {
            Ok(response_form) => Ok(response_form),
            Err(e) => {
                println!("Error: {:?}", e);
                Err(GeminiChatError::ResponseContentError)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    candidates: Vec<Candidate>,
    model_version: Option<String>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: UsageMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    #[serde(rename = "avgLogprobs")]
    avg_logprobs: f64,
    content: Content,
    #[serde(rename = "finishReason")]
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    args: FunctionArgs,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionArgs {
    location: String,
    movie: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageMetadata {
    #[serde(rename = "candidatesTokenCount")]
    candidates_token_count: i32,
    #[serde(rename = "promptTokenCount")]
    prompt_token_count: i32,
    #[serde(rename = "totalTokenCount")]
    total_token_count: i32,
}