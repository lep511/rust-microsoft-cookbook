use crate::compatible::requests::request_chat;
use crate::compatible::utils::GetApiKey;
use crate::compatible::libs::{ChatRequest, Message, ChatResponse};
use crate::llmerror::CompatibleChatError;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatCompatible {
    pub api_key: String,
    pub request: ChatRequest,
    pub timeout: u64,
    pub retry: i32,
    pub url: String,
    pub model: String,
}

#[allow(dead_code)]
impl ChatCompatible {
    pub fn new(url: &str, model: &str) -> Result<Self, CompatibleChatError> {
        let api_key = Self::get_api_key()?;

        let request = ChatRequest {
            model: None,
            messages: None,
            input: None,
            temperature: None,
            max_tokens: None,
            tools: None,
            tool_choice: None,
            frequency_penalty: None,
            presence_penalty: None,
            top_p: None,
            top_k: None,
            stop: None,
            n_completion: None,
            stream: Some(false),
        };
        
        Ok(Self {
            api_key: api_key,
            request: request,
            timeout: 15 * 60, // default: 15 minutes
            retry: 3,         // default: 3 times
            url: url.to_string(),
            model: model.to_string(),
        })
    }

    pub async fn invoke(
        mut self,
        prompt: &str,
    ) -> Result<ChatResponse, CompatibleChatError> {
        
        let new_message = Message {
            role: Some("user".to_string()),
            content: Some(prompt.to_string()),
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self.request.model = Some(self.model.clone());

        let response = match request_chat(
            &self.url,
            &self.request,
            &self.api_key,
            self.timeout,
            self.retry,
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };

        let response_string = response.to_string();
        
        let chat_response: ChatResponse = match serde_json::from_str(&response_string) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            println!("[ERROR] {}", error.message);
            return Err(CompatibleChatError::ResponseContentError);
        } else {
            let format_response = ChatResponse {
                choices: chat_response.choices,
                created: chat_response.created,
                id: chat_response.id,
                model: chat_response.model,
                object: chat_response.object,
                system_fingerprint: chat_response.system_fingerprint,
                usage: chat_response.usage,
                chat_history: self.request.messages,
                error: None,
            };
            Ok(format_response)
        }
    }

    pub async fn with_input_replicate(
        mut self, 
        input: serde_json::Value,
    ) -> Result<serde_json::Value, CompatibleChatError> {
        let model = self.request.model.unwrap_or("".to_string());
        let url_format = format!("{}/{}", self.url, model);
        self.url = url_format;
       
        let request = ChatRequest {
            model: None,
            messages: None,
            input: Some(input),
            temperature: None,
            max_tokens: None,
            tools: None,
            tool_choice: None,
            frequency_penalty: None,
            presence_penalty: None,
            top_p: None,
            top_k: None,
            stop: None,
            n_completion: None,
            stream: None,
        };

        self.request = request;

        let api_key_format = format!("Bearer {}", self.api_key);
    
        let response: serde_json::Value = match request_chat(
            &self.url,
            &self.request,
            &api_key_format,
            self.timeout,
            self.retry,
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };
        
        Ok(response)
    }

    pub async fn baseten_invoke(
        mut self, 
        prompt: &str,
    ) -> Result<serde_json::Value, CompatibleChatError> {
        let new_message = Message {
            role: Some("user".to_string()),
            content: Some(prompt.to_string()),
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }
        
        self.request.stream = Some(false);
        let api_key_format = format!("Api-Key {}", self.api_key);
           
        let response: serde_json::Value = match request_chat(
            &self.url,
            &self.request,
            &api_key_format,
            self.timeout,
            self.retry,
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };
        
        Ok(response)
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature);
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.request.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.request.presence_penalty = Some(presence_penalty);
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.request.top_p = Some(top_p);
        self.request.temperature = None;
        self
    }

    pub fn with_n_completion(mut self, n_completion: u32) -> Self {
        self.request.n_completion = Some(n_completion);
        self
    }

    pub fn with_stop(mut self, stop: Vec<String>) -> Self {
        self.request.stop = Some(stop);
        self
    }

    pub fn with_retry(mut self, retry: i32) -> Self {
        self.retry = retry;
        self
    }

    pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {
        let new_message = Message {
            role: Some("system".to_string()),
            content: Some(system_prompt.to_string()),
        };

        if let Some(messages) = &mut self.request.messages {
            messages.insert(0, new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    pub fn with_assistant_response(mut self,  assistant_response: &str) -> Self {
        let new_message = Message {
            role: Some("assistant".to_string()),
            content: Some(assistant_response.to_string()),
        };

        if let Some(messages) = &mut self.request.messages {
            messages.insert(0, new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    pub fn with_chat_history(mut self, history: Vec<Message>) -> Self {
        self.request.messages = Some(history);
        self
    }

    pub fn with_tools(mut self, tools_data: Vec<serde_json::Value>) -> Self {
        self.request.tools = Some(tools_data);
        self
    }
    
    pub fn with_tool_choice(mut self, tool_choice: serde_json::Value) -> Self {
        self.request.tool_choice = Some(tool_choice);
        self
    }
}

impl GetApiKey for ChatCompatible {}