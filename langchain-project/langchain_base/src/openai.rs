use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::env;

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatOpenAI {
    pub api_key: String,
    pub request: ChatRequest,
    pub timeout: u64,
    pub client: Client,
}

#[allow(dead_code)]
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

        let dev_prompt = "You are a helpful assistant.".to_string();

        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(dev_prompt),
            source: None,
        }];

        let messages = vec![Message {
            role: Role::Developer,
            content: content.clone(),
            recipient: None,
            end_turn: None,
        }];

        let request = ChatRequest {
            model: model.to_string(),
            messages: messages.clone(),
            temperature: Some(0.9),
            max_tokens: Some(1024),
        };
        
        Ok(Self {
            api_key: api_key,
            request: request,
            timeout: 15 * 60, // default: 15 minutes
            client: Client::builder()
                .use_rustls_tls()
                .build()?,
        })
    }

    pub async fn invoke(
        mut self,
        prompt: &str,
    ) -> Result<ChatResponse, OpenAIChatError> {
        
        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(prompt.to_string()),
            source: None,
        }];
        self.request.messages.push(Message {
            role: Role::User,
            content: content.clone(),
            recipient: None,
            end_turn: None,
        });
        let response = self
            .client
            .post(Self::OPENAI_BASE_URL)
            .timeout(Duration::from_secs(self.timeout))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&self.request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
       
        let response = response.to_string();
        let chat_response: ChatResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(OpenAIChatError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            println!("[ERROR] {}", error.message);
            return Err(OpenAIChatError::ResponseContentError);
        } else {
            let format_response = ChatResponse {
                choices: chat_response.choices,
                created: chat_response.created,
                id: chat_response.id,
                model: chat_response.model,
                object: chat_response.object,
                system_fingerprint: chat_response.system_fingerprint,
                usage: chat_response.usage,
                chat_history: Some(self.request.messages),
                error: None,
            };
            Ok(format_response)
        }
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature);
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {
        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(system_prompt.to_string()),
            source: None,
        }];
        self.request.messages[0].content = content;
        self
    }

    pub fn with_assistant_response(mut self,  assistant_response: &str) -> Self {
        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(assistant_response.to_string()),
            source: None,
        }];
        self.request.messages.push(Message {
            role: Role::Assistant,
            content: content.clone(),
            recipient: None,
            end_turn: None,
        });
        self
    }

    pub fn with_chat_history(mut self, history: Vec<Message>) -> Self {
        self.request.messages = history;
        self
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Platform,
    Developer,
    User,
    Assistant,
    Tool,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: Vec<InputContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_turn: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputContent {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Source>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Source {
    #[serde(rename = "type")]
    source_type: String,
    media_type: String,
    data: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Option<Vec<Choice>>,
    pub created: Option<u64>,
    pub id: Option<String>,
    pub model: Option<String>,
    pub object: Option<String>,
    pub system_fingerprint: Option<String>,
    pub usage: Option<Usage>,
    pub chat_history: Option<Vec<Message>>,
    pub error: Option<ErrorDetails>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub finish_reason: String,
    pub index: u32,
    pub logprobs: Option<serde_json::Value>,
    pub message: ChatMessage,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    pub content: String,
    pub refusal: Option<String>,
    pub role: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Usage {
    pub completion_tokens: u32,
    pub completion_tokens_details: CompletionTokensDetails,
    pub prompt_tokens: u32,
    pub prompt_tokens_details: PromptTokensDetails,
    pub total_tokens: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CompletionTokensDetails {
    pub accepted_prediction_tokens: u32,
    pub audio_tokens: u32,
    pub reasoning_tokens: u32,
    pub rejected_prediction_tokens: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PromptTokensDetails {
    pub audio_tokens: u32,
    pub cached_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    pub param: Option<String>,
    #[serde(rename = "type")]
    pub error_type: String,
}