use crate::anthropic::libs::{
    ChatRequest, Content, Message, ChatResponse,
    Source,
};
use crate::anthropic::utils::{
    GetApiKey, read_file_data,
};
use crate::anthropic::requests::request_chat;
use crate::anthropic::error::AnthropicError;
use serde_json::Value;
use std::time::Duration;
use log::error;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatAnthropic {
    pub api_key: String,
    pub request: ChatRequest,
    pub timeout: Duration,
    pub max_retries: u32,
}

#[allow(dead_code)]
impl ChatAnthropic {
    pub fn new(model: &str) -> Self {
        let api_key: String = match Self::get_api_key() {
            Ok(api_key) => api_key,
            Err(_) => "not_key".to_string()
        };

        let request = ChatRequest {
            model: model.to_string(),
            messages: None,
            system: None,
            temperature: None,
            max_tokens: Some(1024),
            tools: None,
            tool_choice: None,
            stream: false,
        };
        
        Self {
            api_key: api_key,
            request: request,
            timeout: Duration::from_secs(300), // default: 5 minutes
            max_retries: 3,         // default: 3 times
        }
    }

    pub async fn invoke(
        mut self,
        prompt: &str,
    ) -> Result<ChatResponse, AnthropicError> {

        let content = vec![Content {
            content_type: "text".to_string(),
            text: Some(prompt.to_string()),
            source: None,
            image_url: None,
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];

        let new_message = Message {
            role: "user".to_string(),
            content: content,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        match self.send_request().await {
            Ok(response) => Ok(response),
            Err(e) => Err(e),
        }
    }

    pub async fn with_tool_result(
        mut self, 
        tool_id: &str, 
        content: &str
    ) -> Result<ChatResponse, AnthropicError> {
        let content = vec![Content {
            content_type: "tool_result".to_string(),
            text: None,
            source: None,
            image_url: None,
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: Some(content.to_string()),
            tool_use_id: Some(tool_id.to_string()),
        }];

        let new_message = Message {
            role: "user".to_string(),
            content: content,
        };
        
        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        match self.send_request().await {
            Ok(response) => Ok(response),
            Err(e) => Err(e),
        }
    }

    pub async fn send_request(self) -> Result<ChatResponse, AnthropicError> {
        let response: String = match request_chat(
            &self.request,
            &self.api_key,
            self.timeout,
            self.max_retries,
        ).await {
            Ok(response) => response,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(AnthropicError::ResponseContentError);
            }
        };

        let chat_response: ChatResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(AnthropicError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            error!("Error {}", error.message);
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
                chat_history: self.request.messages.clone(),
                usage: chat_response.usage,
                error: None,
            };

            Ok(format_response)
        }
    }

    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = Duration::from_secs(timeout);
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

    pub fn with_tools(mut self, tools: Vec<Value>, tool_choice: Option<Value>) -> Self {
        // https://docs.anthropic.com/en/docs/build-with-claude/tool-use#controlling-claudes-output
        self.request.tools = Some(tools);
        self.request.tool_choice = tool_choice;
        self
    }

    pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {
        self.request.system = Some(system_prompt.to_string());
        self
    }

    pub fn with_max_retries(mut self, retry: u32) -> Self {
        self.max_retries = retry;
        self
    }

    pub fn with_assistant_content(mut self,  assistant_content: Vec<Content>) -> Self {
        let new_message = Message {
            role: "assistant".to_string(),
            content: assistant_content,
        };
        
        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    pub fn with_assistant_response(mut self,  assistant_response: &str) -> Self {
        let content = vec![Content {
            content_type: "text".to_string(),
            text: Some(assistant_response.to_string()),
            source: None,
            image_url: None,
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];

        let new_message = Message {
            role: "assistant".to_string(),
            content: content,
        };
        
        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    pub fn with_chat_history(mut self, history: Vec<Message>) -> Self {
        self.request.messages = Some(history);
        self
    }

    pub fn with_image_base64(
        mut self, 
        image_base64: &str, 
        mime_type: &str
    ) -> Self {

        let content = vec![Content {
            content_type: "image".to_string(),
            text: None,
            source: Some(Source {
                source_type: "base64".to_string(),
                media_type: mime_type.to_string(),
                data: image_base64.to_string(),
            }),
            image_url: None,
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];

        let new_message = Message {
            role: "user".to_string(),
            content: content,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    pub fn with_image_file(
        mut self, 
        file_path: &str, 
        mime_type: &str
    ) -> Self {

        let image_base64 = match read_file_data(file_path) {
            Ok(data) => data,
            Err(e) => {
                error!("Error {:?}", e);
                return self;
            }
        };
        
        let content = vec![Content {
            content_type: "image".to_string(),
            text: None,
            source: Some(Source {
                source_type: "base64".to_string(),
                media_type: mime_type.to_string(),
                data: image_base64,
            }),
            image_url: None,
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];

        let new_message = Message {
            role: "user".to_string(),
            content: content,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }
}

impl GetApiKey for ChatAnthropic {}