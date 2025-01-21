use crate::anthropic::libs::{
    ChatRequest, InputContent, Message, ChatResponse,
    Source,
};
use crate::anthropic::MIME_TYPE_SUPPORTED;
use crate::anthropic::utils::GetApiKey;
use crate::anthropic::requests::request_chat;
use crate::llmerror::AnthropicError;
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatAnthropic {
    pub api_key: String,
    pub request: ChatRequest,
    pub timeout: u64,
    pub retry: i32,
}

#[allow(dead_code)]
impl ChatAnthropic {
    pub fn new(model: &str) -> Result<Self, AnthropicError> {
        let api_key = Self::get_api_key()?;

        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some("Init message.".to_string()),
            source: None,
            image_url: None,
            image_base64: None,
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
            retry: 3,         // default: 3 times
        })
    }

    pub async fn invoke(
        mut self,
        prompt: &str,
    ) -> Result<ChatResponse, AnthropicError> {
        if let Some(content) = &mut self.request.messages[0].content {
            if content[0].text == Some("Init message.".to_string()) {
                content[0].text = Some(prompt.to_string());
            } else {
                let content = vec![InputContent {
                    content_type: "text".to_string(),
                    text: Some(prompt.to_string()),
                    source: None,
                    image_url: None,
                    image_base64: None,
                }];
                self.request.messages.push(Message {
                    role: Some("user".to_string()),
                    content: Some(content.clone()),
                });
            }
        };

        let response: String = match request_chat(
            &self.request,
            &self.api_key,
            self.timeout,
            self.retry,
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(AnthropicError::ResponseContentError);
            }
        };

        let chat_response: ChatResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(AnthropicError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            println!("[ERROR] {}", error.message);
            return Err(AnthropicError::ResponseContentError);
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

    pub fn with_retry(mut self, retry: i32) -> Self {
        self.retry = retry;
        self
    }

    pub fn with_assistant_response(mut self,  assistant_response: &str) -> Self {
        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(assistant_response.to_string()),
            source: None,
            image_url: None,
            image_base64: None,
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

    pub fn with_image(
        mut self, 
        image_base64: &str, 
        mime_type: &str
    ) -> Self {

        if !MIME_TYPE_SUPPORTED.contains(&mime_type) {
            println!(
                "[ERROR] Unsupported media type: {}. Supported: {}", 
                mime_type,
                MIME_TYPE_SUPPORTED.join(", ")
            );
            return self;
        }

        let content = vec![InputContent {
            content_type: "image".to_string(),
            text: None,
            source: Some(Source {
                source_type: "base64".to_string(),
                media_type: mime_type.to_string(),
                data: image_base64.to_string(),
            }),
            image_url: None,
            image_base64: None,
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

impl GetApiKey for ChatAnthropic {}