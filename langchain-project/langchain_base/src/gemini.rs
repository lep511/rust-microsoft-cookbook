use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use serde_json::Value;

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub contents: Vec<Content>,
    pub tools: Option<Vec<Value>>,
    #[serde(rename = "generationConfig")]
    pub generation_config: Option<GenerationConfig>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(rename = "functionCall", default)]
    pub function_call: Option<FunctionCall>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatGemini {
    pub base_url: String,
    pub client: Client,
}

#[allow(dead_code)]
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

    pub async fn invoke(&self, contents: ChatRequest) -> Result<ChatResponse, GeminiChatError> {
        let response = self
            .client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .json(&contents)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let response = response.to_string();
        let chat_response: ChatResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(GeminiChatError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            println!("[ERROR] {}", error.message);
            return Err(GeminiChatError::ResponseContentError);
        } else {
            Ok(chat_response)
        }
    }

    // pub fn with_temperature(mut self, temperature: f32) -> Self {
    //     self.temperature = temperature;
    //     self
    // }

    // pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
    //     self.max_tokens = Some(max_tokens);
    //     self
    // }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub candidates: Option<Vec<Candidate>>,
    pub model_version: Option<String>,
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: Option<UsageMetadata>,
    pub error: Option<ErrorDetails>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    #[serde(rename = "avgLogprobs")]
    pub avg_logprobs: Option<f64>,
    pub content: Option<Content>,
    #[serde(rename = "finishReason")]
    pub finish_reason: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub args: String,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct UsageMetadata {
    #[serde(rename = "candidatesTokenCount")]
    pub candidates_token_count: i32,
    #[serde(rename = "promptTokenCount")]
    pub rompt_token_count: i32,
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: i32,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: i32,
    pub message: String,
    pub status: String,
    pub details: Vec<ErrorDetail>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "@type")]
    pub type_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub service: String,
}