use crate::openai::requests::request_chat;
use crate::openai::utils::GetApiKey;
use crate::openai::libs::{
    ChatRequest, InputContent,
    Message, Role, ChatResponse, 
};
use crate::llmerror::OpenAIError;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatOpenAI {
    pub api_key: String,
    pub request: ChatRequest,
    pub timeout: u64,
    pub retry: i32,
}

#[allow(dead_code)]
impl ChatOpenAI {
    pub fn new(model: &str) -> Result<Self, OpenAIError> {
        let api_key = Self::get_api_key()?;

        let request = ChatRequest {
            model: model.to_string(),
            messages: None,
            temperature: None,
            tools: None,
            tool_choice: None,
            max_completion_tokens: None,
            response_format: None,
            frequency_penalty: None,
            presence_penalty: None,
            top_p: None,
            stream: None,
            n_completion: Some(1),
            stop: None,
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
    ) -> Result<ChatResponse, OpenAIError> {
        
        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(prompt.to_string()),
            source: None,
        }];

        let new_message = Message {
            role: Role::User,
            content: content.clone(),
            recipient: None,
            end_turn: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        let response: String = match request_chat(
            &self.request,
            &self.api_key,
            self.timeout,
            self.retry,
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(OpenAIError::ResponseContentError);
            }
        };
        
        let chat_response: ChatResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(OpenAIError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            println!("[ERROR] {}", error.message);
            return Err(OpenAIError::ResponseContentError);
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

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        if temperature < 0.0 || temperature > 2.0 {
            println!(
                "[ERROR] Temperature must be between 0.0 and 2.0. Actual temperature is {}", 
                self.request.temperature.unwrap_or(0.0)
            );
            self
        } else {
            self.request.temperature = Some(temperature);
            self
        }
    }
    
    pub fn with_max_completion_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_completion_tokens = Some(max_tokens);
        self
    }

    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        if frequency_penalty < -2.0 || frequency_penalty > 2.0 {
            println!(
                "[ERROR] Frequency penalty must be between -2.0 and 2.0. Actual frequency penalty is {}",
                self.request.frequency_penalty.unwrap_or(0.0)
            );
            self
        } else {
            self.request.frequency_penalty = Some(frequency_penalty);
            self
        }
    }

    pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
        if presence_penalty < -2.0 || presence_penalty > 2.0 {
            println!(
                "[ERROR] Presence penalty must be between -2.0 and 2.0. Actual presence penalty is {}",
                self.request.presence_penalty.unwrap_or(0.0)
            );
            self
        } else {
            self.request.presence_penalty = Some(presence_penalty);
            self
        }
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        if top_p < 0.0 || top_p > 1.0 {
            println!(
                "[ERROR] Top p must be between 0.0 and 1.0. Actual top p is {}",
                self.request.top_p.unwrap_or(0.0)
            );
            self
        } else {
            self.request.top_p = Some(top_p);
            self
        }
    }

    pub fn with_n_completion(mut self, n_completion: u32) -> Self {
        self.request.n_completion = Some(n_completion);
        self
    }

    pub fn with_stop(mut self, stop: Vec<String>) -> Self {
        self.request.stop = Some(stop);
        self
    }

    pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {
        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(system_prompt.to_string()),
            source: None,
        }];

        let new_message = Message {
            role: Role::Developer,
            content: content.clone(),
            recipient: None,
            end_turn: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.insert(0, new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    pub fn with_assistant_response(mut self,  assistant_response: &str) -> Self {
        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(assistant_response.to_string()),
            source: None,
        }];

        let new_message = Message {
            role: Role::Assistant,
            content: content.clone(),
            recipient: None,
            end_turn: None,
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

    pub fn with_tools(mut self, tools_data: Vec<serde_json::Value>) -> Self {
        self.request.tools = Some(tools_data);
        self
    }

    pub fn with_tool_choice(mut self, tool_choice: serde_json::Value) -> Self {
        self.request.tool_choice = Some(tool_choice);
        self
    }

    pub fn with_retry(mut self, retry: i32) -> Self {
        self.retry = retry;
        self
    }
}

impl GetApiKey for ChatOpenAI {}