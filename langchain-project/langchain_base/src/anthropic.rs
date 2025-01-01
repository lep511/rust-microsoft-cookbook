use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use crate::llmerror::AnthropicChatError;
use std::env;

pub static ANTHROPIC_BASE_URL: &str = "https://api.anthropic.com/v1/messages";

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
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

        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some("Init message.".to_string()),
            source: None,
        }];

        let messages = vec![Message {
            role: Some("user".to_string()),
            content: Some(content.clone()),
        }];

        let request = ChatRequest {
            model: model.to_string(),
            messages: messages.clone(),
            system: None,
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
        if let Some(content) = &mut self.request.messages[0].content {
            if content[0].text == Some("Init message.".to_string()) {
                content[0].text = Some(prompt.to_string());
            } else {
                let content = vec![InputContent {
                    content_type: "text".to_string(),
                    text: Some(prompt.to_string()),
                    source: None,
                }];
                self.request.messages.push(Message {
                    role: Some("user".to_string()),
                    content: Some(content.clone()),
                });
            }
        };

        let response = self
            .client
            .post(ANTHROPIC_BASE_URL)
            .timeout(Duration::from_secs(self.timeout))
            .header("x-api-key", self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&self.request)
            .send()
            .await?
            .json::<Value>()
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
            let format_response: ChatResponse = ChatResponse {
                id: chat_response.id,
                content: chat_response.content,
                model: chat_response.model,
                role: chat_response.role,
                stop_reason: chat_response.stop_reason,
                stop_sequence: chat_response.stop_sequence,
                response_type: chat_response.response_type,
                chat_history: Some(self.request.messages.clone()),
                usage: chat_response.usage,
                error: None,
            };

            Ok(format_response)
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
        self.request.system = Some(system_prompt.to_string());
        self
    }

    pub fn with_assistant_response(mut self,  assistant_response: &str) -> Self {
        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(assistant_response.to_string()),
            source: None,
        }];

        self.request.messages.push(
            Message {
                role: Some("assistant".to_string()),
                content: Some(content),
            }
        );
        self
    }

    pub fn with_chat_history(mut self, history: Vec<Message>) -> Self {
        self.request.messages = history;
        self
    }

    pub fn with_image_gif(mut self, image: &str) -> Self {
        let content = vec![InputContent {
            content_type: "image".to_string(),
            text: None,
            source: Some(Source {
                source_type: "base64".to_string(),
                media_type: "image/gif".to_string(),
                data: image.to_string(),
            }),
        }];

        self.request.messages.push(
            Message {
                role: Some("user".to_string()),
                content: Some(content),
            }
        );
        self
    }

    pub fn with_image_png(mut self, image: &str) -> Self {
        let content = vec![InputContent {
            content_type: "image".to_string(),
            text: None,
            source: Some(Source {
                source_type: "base64".to_string(),
                media_type: "image/png".to_string(),
                data: image.to_string(),
            }),
        }];
        self.request.messages.push(
            Message {
                role: Some("user".to_string()),
                content: Some(content),
            }
        );
        self
    }

    pub fn with_image_jpeg(mut self, image: &str) -> Self {
        let content = vec![InputContent {
            content_type: "image".to_string(),
            text: None,
            source: Some(Source {
                source_type: "base64".to_string(),
                media_type: "image/jpeg".to_string(),
                data: image.to_string(),
            }),
        }];
        self.request.messages.push(
            Message {
                role: Some("user".to_string()),
                content: Some(content),
            }
        );
        self
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Option<String>,
    pub content: Option<Vec<InputContent>>,
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
    pub content: Option<Vec<Content>>,
    pub id: Option<String>,
    pub model: Option<String>,
    pub role: Option<String>,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    #[serde(rename = "type")]
    pub response_type: String,
    pub usage: Option<Usage>,
    pub chat_history: Option<Vec<Message>>,
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
