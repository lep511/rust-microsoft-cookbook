use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use std::env;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum AnthropicChatError {
    #[error("ANTHROPIC API key not found in environment variables")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,
    pub stream: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatAnthropic {
    pub api_key: String,
    pub request: ChatRequest,
    pub timeout: u64,
    pub client: Client,
}

#[allow(dead_code)]
impl ChatAnthropic {
    const ANTHROPIC_BASE_URL: &'static str = "https://api.anthropic.com/v1/messages";

    pub fn new(model: &str) -> Result<Self, AnthropicChatError> {
        let api_key = match env::var("ANTHROPIC_API_KEY") {
            Ok(key) => key,
            Err(env::VarError::NotPresent) => {
                return Err(AnthropicChatError::ApiKeyNotFound);
            }
            Err(e) => {
                return Err(AnthropicChatError::EnvError(e));
            }
        };

        let messages = vec![Message {
            role: Some("user".to_string()),
            content: Some("Hello!".to_string()),
        }];

        let request = ChatRequest {
            model: model.to_string(),
            messages: messages.clone(),
            temperature: Some(0.9),
            max_tokens: Some(1024),
            tools: None,
            tool_choice: None,
            stream: false,
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
    ) -> Result<ChatResponse, AnthropicChatError> {
        
        let total_msg = self.request.messages.len() - 1;
        self.request.messages[total_msg].content = Some(prompt.to_string());   
        let response = self
            .client
            .post(Self::ANTHROPIC_BASE_URL)
            .timeout(Duration::from_secs(self.timeout))
            .header("x-api-key", self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
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
                return Err(AnthropicChatError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            println!("[ERROR] {}", error.message);
            return Err(AnthropicChatError::ResponseContentError);
        } else {
            Ok(chat_response)
        }
    }

    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature);
        self
    }

    pub fn with_stream(mut self, stream: bool) -> Self {
        self.request.stream = stream;
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_tools(mut self, tools: Vec<Value>, tool_choice: Value) -> Self {
        // https://docs.anthropic.com/en/docs/build-with-claude/tool-use#controlling-claudes-output
        self.request.tools = Some(tools);
        self.request.tool_choice = Some(tool_choice);
        self
    }

    pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {
        self.request.messages.insert(
            0,
            Message {
                role: Some("system".to_string()),
                content: Some(system_prompt.to_string()),
            },
        );
        self
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub content: Option<Vec<Content>>,
    pub id: Option<String>,
    pub model: Option<String>,
    pub role: Option<String>,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    #[serde(rename = "type")]
    response_type: String,
    pub usage: Option<Usage>,
    pub error: Option<ErrorDetails>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Content {
    pub id: Option<String>,
    pub input: Option<Value>,
    pub name: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "type")]
    pub content_type: Option<String>,
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
    pub cache_creation_input_tokens: u32,
    pub cache_read_input_tokens: u32,
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
}