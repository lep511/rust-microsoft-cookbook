use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Clone)]
pub struct ChatOpenAI {
    api_key: String,
    model: String,
    temperature: f32,
    max_tokens: Option<u32>,
    client: Client,
}

#[derive(Debug, thiserror::Error)]
pub enum ChatError {
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

    pub fn new() -> Result<Self, ChatError> {
        let api_key = match env::var("OPENAI_API_KEY") {
            Ok(key) => key,
            Err(env::VarError::NotPresent) => {
                return Err(ChatError::ApiKeyNotFound);
            }
            Err(e) => {
                return Err(ChatError::EnvError(e));
            }
        };
        
        Ok(Self {
            api_key,
            model: String::from("egpt-4o-mini"),
            temperature: 0.7,
            max_tokens: None,
            client: Client::builder()
                .use_rustls_tls()
                .build()?,
        })
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub async fn chat(
        &self,
        messages: Vec<Message>,
    ) -> Result<String, ChatError> {
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

        if response["choices"][0]["finish_reason"] == "stop" {
            let content = response["choices"][0]["message"]["content"]
                .as_str()
                .ok_or(ChatError::ResponseContentError)?;
            return Ok(content.to_string());
        } else {
            println!("Error E108: {}", response["error"]["message"].to_string());
            return Err(ChatError::ResponseContentError);
        }
    }
}

// Example usage
async fn example() -> Result<(), ChatError> {
    // Initialize with API key from environment variable
    let chat = ChatOpenAI::new()?;
    
    let messages = vec![Message {
        role: "user".to_string(),
        content: "Write a haiku about ai".to_string(),
    }];

    let response = chat.chat(messages).await?;
    println!("Response: {}", response);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match example().await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{}", e);
            Err(e.into())
        }
    }
}
