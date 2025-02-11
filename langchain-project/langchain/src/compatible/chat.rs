use futures::pin_mut;
use futures::StreamExt;
use async_stream::stream;
use log::{info, error};
use crate::compatible::requests::{
    request_chat, get_request, strem_chat,
};
use crate::compatible::utils::{GetApiKey, read_file_data};
use crate::compatible::libs::{
    ChatRequest, Message, ChatResponse, ChatStreamResponse, 
    Content, ImageUrl,
};
use crate::llmerror::CompatibleChatError;
use tokio::time::sleep;
use std::time::Duration;
use serde_json::json;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatCompatible {
    pub api_key: String,
    pub request: ChatRequest,
    pub timeout: u64,
    pub max_retries: i32,
    pub url: String,
    pub model: String,
}

#[allow(dead_code)]
impl ChatCompatible {
    pub fn new(url: &str, model: &str) -> Self {
        let api_key: String = match Self::get_api_key() {
            Ok(api_key) => api_key,
            Err(_) => "not_key".to_string()
        };

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
            min_p: None,
            top_k: None,
            stop: None,
            n_completion: None,
            response_format: None,
            stream: Some(false),
        };
        
        Self {
            api_key: api_key,
            request: request,
            timeout: 15 * 60, // default: 15 minutes
            max_retries: 3,         // default: 3 times
            url: url.to_string(),
            model: model.to_string(),
        }
    }

    pub async fn invoke(
        mut self,
        prompt: &str,
    ) -> Result<ChatResponse, CompatibleChatError> {

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
            role: Some("user".to_string()),
            content: content,
            tool_calls: None,
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
            self.max_retries,
        ).await {
            Ok(response) => response,
            Err(error) => {
                error!("Error {:?}", error);
                return Err(error);
            }
        };

        let response_string = response.to_string();
        
        let chat_response: ChatResponse = match serde_json::from_str(&response_string) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            error!("Error {}", error.message);
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
        let url_format = format!("{}/{}", self.url, self.model);
       
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
            min_p: None,
            top_k: None,
            stop: None,
            n_completion: None,
            response_format: None,
            stream: None,
        };

        self.request = request;
    
        let response: serde_json::Value = match request_chat(
            &url_format,
            &self.request,
            &self.api_key,
            self.timeout,
            self.max_retries,
        ).await {
            Ok(response) => response,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };

        let url_str = response
            .get("urls")
            .and_then(|urls| urls.get("get"))
            .and_then(|url| url.as_str())
            .ok_or(CompatibleChatError::ResponseContentError)?;

        let mut status = String::from("processing");

        while status == "processing" {
            let response = get_request(url_str, &self.api_key)
                .await
                .map_err(|e| {
                    error!("Error fetching response: {:?}", e);
                    CompatibleChatError::ResponseContentError
                })?;
    
            status = response
                .get("status")
                .and_then(|s| s.as_str())
                .unwrap_or("error")
                .to_string();
    
            if status == "processing" {
                info!("Status: {}", status);
                sleep(Duration::from_secs(4)).await;
            } else {
                return Ok(response);
            }
        }
    
        Ok(json!({"detail": "Not found"}))
    }
    
    pub async fn baseten_invoke(
        mut self, 
        prompt: &str,
    ) -> Result<serde_json::Value, CompatibleChatError> {
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
            role: Some("user".to_string()),
            content: content,
            tool_calls: None,
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
            self.max_retries,
        ).await {
            Ok(response) => response,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };
        
        Ok(response)
    }

    /// Retrieves model information by making a GET request to the specified endpoint
    /// 
    /// # Arguments
    /// 
    /// * `self` - The instance containing base URL and API key configuration
    /// * `suffix_url` - The URL path suffix to append to the base URL
    /// 
    /// # Returns
    /// 
    /// * `Result<serde_json::Value, CompatibleChatError>` - JSON response on success, or error on failure
    /// 
    /// # Errors
    /// 
    /// Returns `CompatibleChatError::ResponseContentError` if the request fails or response cannot be parsed
    ///
    pub async fn get_models(
        self, 
        suffix_url: &str
    ) -> Result<serde_json::Value, CompatibleChatError> {
        let url = format!("{}/{}", self.url, suffix_url);

        let response: serde_json::Value = match get_request(
            &url,
            &self.api_key,
        ).await {
            Ok(response) => response,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(CompatibleChatError::ResponseContentError);
            }
        };

        Ok(response)
    }

    pub fn stream_response(
        mut self,
        prompt: String,  // Don't change type for stream
    ) -> impl futures::Stream<Item = ChatStreamResponse> {
        stream! {            
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
                role: Some("user".to_string()),
                content: content,
                tool_calls: None,
            };
    
            if let Some(messages) = &mut self.request.messages {
                messages.push(new_message);
            } else {
                self.request.messages = Some(vec![new_message]);
            }

            self.request.model = Some(self.model.clone());
            self.request.stream = Some(true);

            let stream = strem_chat(
                self.url.clone(),
                self.api_key.clone(),
                self.request.clone(),
            );

            pin_mut!(stream);

            while let Some(chat_response) = stream.next().await {
                yield chat_response;
            }
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
        self
    }

    pub fn with_min_p(mut self, min_p: f32) -> Self {
        self.request.min_p = Some(min_p);
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

    pub fn with_max_retries(mut self, max_retries: i32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {

        let content = vec![Content {
            content_type: "text".to_string(),
            text: Some(system_prompt.to_string()),
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
            role: Some("system".to_string()),
            content: content,
            tool_calls: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.insert(0, new_message);
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
            role: Some("assistant".to_string()),
            content: content,
            tool_calls: None,
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

    pub fn with_image_url(
        mut self, 
        image_url: &str, 
    ) -> Self {

        let image_url = ImageUrl {
            url: image_url.to_string(),
            detail: "high".to_string(),
        };

        let content = vec![Content {
            content_type: "image_url".to_string(),
            text: None,
            source: None,
            image_url: Some(image_url),
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];

        let new_message = Message {
            role: Some("user".to_string()),
            content: content,
            tool_calls: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    pub fn with_image_base64(
        mut self, 
        image_base64: &str, 
        mime_type: &str
    ) -> Self {

        let url = format!("data:{};base64,{}", mime_type, image_base64);

        let image_url = ImageUrl {
            url: url,
            detail: "high".to_string(),
        };
        
        let content = vec![Content {
            content_type: "image_url".to_string(),
            text: None,
            source: None,
            image_url: Some(image_url),
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];

        let new_message = Message {
            role: Some("user".to_string()),
            content: content,
            tool_calls: None,
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

        let url = format!("data:{};base64,{}", mime_type, image_base64);

        let image_url = ImageUrl {
            url: url,
            detail: "high".to_string(),
        };
        
        let content = vec![Content {
            content_type: "image_url".to_string(),
            text: None,
            source: None,
            image_url: Some(image_url),
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        }];

        let new_message = Message {
            role: Some("user".to_string()),
            content: content,
            tool_calls: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }
}

impl GetApiKey for ChatCompatible {}