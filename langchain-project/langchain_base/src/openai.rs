use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ChatOpenAI {
    pub api_key: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub client: Client,
}

#[derive(Debug, thiserror::Error)]
pub enum OpenAIChatError {
    #[error("OpenAI API key not found in environment variables")]
    ApiKeyNotFound,
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    #[error("Failed to get response content")]
    ResponseContentError,
}

impl ChatOpenAI {
    const OPENAI_BASE_URL: &'static str = "https://api.openai.com/v1/chat/completions";

    pub fn new(model: &str) -> Result<Self, OpenAIChatError> {
        let api_key = match env::var("OPENAI_API_KEY") {
            Ok(key) => key,
            Err(env::VarError::NotPresent) => {
                return Err(OpenAIChatError::ApiKeyNotFound);
            }
            Err(e) => {
                return Err(OpenAIChatError::EnvError(e));
            }
        };
        
        Ok(Self {
            api_key,
            model: model.to_string(),
            temperature: 0.7,
            max_tokens: 1024,
            client: Client::builder()
                .use_rustls_tls()
                .build()?,
        })
    }

    pub async fn invoke(
        &self,
        messages: Vec<Message>,
    ) -> Result<String, OpenAIChatError> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };

        let response = self
            .client
            .post(Self::OPENAI_BASE_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response["choices"][0]["message"]["content"]
            .as_str()
            .ok_or(OpenAIChatError::ResponseContentError)?
            .to_string())
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Response {
//     candidates: Vec<Candidate>,
//     model_version: Option<String>,
//     #[serde(rename = "usageMetadata")]
//     usage_metadata: UsageMetadata,
// }