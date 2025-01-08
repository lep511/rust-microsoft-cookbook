use crate::llmerror::GeminiError;
use generation_config::GenerationConfig;
use gemini_utils::{GetApiKey, TaskType};
use gemini_libs::{
    ChatRequest, Content, Part, FileData, EmbedResponse,
    InlineData, ChatResponse, EmbedRequest,
};
use gemini_requests::{
    request_chat, request_media, request_cache,
    request_embed,
};

pub mod generation_config;
pub mod gemini_libs;
pub mod gemini_utils;
pub mod gemini_requests;
pub mod error_detail;

pub static GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
pub static UPLOAD_BASE_URL: &str = "https://generativelanguage.googleapis.com/upload/v1beta";

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatGemini {
    pub base_url: String,
    pub model: String,
    pub request: ChatRequest,
    pub timeout: u64,
}

#[allow(dead_code)]
impl ChatGemini {
    pub fn new(model: &str) -> Result<Self, GeminiError> {
        let api_key = Self::get_api_key()?;
        
        let base_url = format!(
            "{}/models/{}:generateContent?key={}",
            GEMINI_BASE_URL,
            model,
            api_key,
        );

        let request = ChatRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: Some("Init message.".to_string()),
                    function_call: None,
                    inline_data: None,
                    file_data: None,
                }],
            }],
            tools: None,
            tool_config: None,
            system_instruction: None,
            cached_content: None,
            generation_config: Some(GenerationConfig {
                temperature: Some(0.9),
                top_k: Some(40),
                top_p: Some(0.95),
                max_output_tokens: Some(2048),
                response_mime_type: Some("text/plain".to_string()),
                response_schema: None,
                stop_sequences: None,
                candidate_count: Some(1),
                presence_penalty: None,
                frequency_penalty: None,
                response_logprobs: None,
                log_probs: None,
            }),
        };
        
        Ok(Self {
            base_url: base_url,
            model: model.to_string(),
            request: request,
            timeout: 15 * 60, // default: 15 minutes
        })
    }

    pub async fn invoke(mut self, prompt: &str) -> Result<ChatResponse, GeminiError> {

        if self.request.contents[0].parts[0].text == Some("Init message.".to_string()) {
            self.request.contents[0].parts[0].text = Some(prompt.to_string());
        } else {
            let content = Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: Some(prompt.to_string()),
                    function_call: None,
                    inline_data: None,
                    file_data: None,
                }],
            };
            self.request.contents.push(content);
        }
        
        let response = match request_chat(
            &self.base_url,
            &self.request,
            self.timeout
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(GeminiError::RequestChatError);
            }
        };
 
        let chat_response: ChatResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(GeminiError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            println!("[ERROR] {:?}", error);
            return Err(GeminiError::ResponseContentError);
        } else {
            let format_response = ChatResponse {
                candidates: chat_response.candidates,
                model_version: chat_response.model_version,
                usage_metadata: chat_response.usage_metadata,
                chat_history: Some(self.request.contents.clone()),
                error: None,
            };
            Ok(format_response)
        }
    }

    pub async fn media_upload(mut self, img_path: &str, mime_type: &str) 
    -> Result<Self, GeminiError> {
        let api_key = Self::get_api_key()?;
        let upload_url = format!(
            "{}/files?key={}",
            UPLOAD_BASE_URL,
            api_key
        );      
      
        let file_uri = match request_media(
            &upload_url, 
            img_path, 
            mime_type
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(GeminiError::RequestUploadError);
            }
        };
        
        let content = Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: None,
                function_call: None,
                inline_data: None,
                file_data: Some(FileData {
                    mime_type: mime_type.to_string(),
                    file_uri: file_uri,
                }),
            }]
        };
        self.request.contents.push(content);

        Ok(
            Self{
                base_url: self.base_url, 
                model: self.model, 
                request: self.request, 
                timeout: self.timeout,
            }
        )
    }

    pub async fn cache_upload(
        self, 
        data: String, 
        mime_type: &str, 
        instruction: &str,
        ttl: u32,
    ) -> Result<String, GeminiError> {
        let api_key = Self::get_api_key()?;
        let url_cache = format!(
            "{}/cachedContents?key={}", 
            GEMINI_BASE_URL,
            api_key
        );

        let cache_name = match request_cache(
            url_cache,
            data,
            mime_type.to_string(),
            instruction.to_string(),
            &self.model,
            ttl,
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(GeminiError::RequestCacheError);
            }
        }; 

        Ok(cache_name)
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        match &mut self.request.generation_config {
            Some(config) => {
                config.temperature = Some(temperature);
            }
            None => ()
        };
        self
    }

    pub fn with_top_k(mut self, top_k: u32) -> Self {
        match &mut self.request.generation_config {
            Some(config) => {
                config.top_k = Some(top_k);
            }
            None => ()
        };
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        match &mut self.request.generation_config {
            Some(config) => {
                config.top_p = Some(top_p);
            }
            None => ()
        };
        self
    }

    pub fn with_candidate_count(mut self, candidate_count: u32) -> Self {
        match &mut self.request.generation_config {
            Some(config) => {
                config.candidate_count = Some(candidate_count);
            }
            None => ()
        };
        self
    }

    pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        match &mut self.request.generation_config {
            Some(config) => {
                config.stop_sequences = Some(stop_sequences);
            }
            None => ()
        };
        self
    }

    pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {
        self.request.system_instruction = Some(Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: Some(system_prompt.to_string()),
                function_call: None,
                inline_data: None,
                file_data: None,
            }],
        });
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        match &mut self.request.generation_config {
            Some(config) => {
                config.max_output_tokens = Some(max_tokens);
            }
            None => ()
        };
        self
    }

    pub fn with_response_schema(mut self, response_schema: serde_json::Value) -> Self {
        match &mut self.request.generation_config {
            Some(config) => {
                config.response_schema = Some(response_schema);
                config.response_mime_type = Some("application/json".to_string());
            }
            None => ()
        };
        self
    }
    
    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_assistant_response(mut self,  assistant_response: &str) -> Self {
        let content = Content {
            role: "model".to_string(),
            parts: vec![Part {
                text: Some(assistant_response.to_string()),
                function_call: None,
                inline_data: None,
                file_data: None,
            }],
        };
        self.request.contents.push(content);
        self
    }

    pub fn with_cached_content(mut self, cache_name: String) -> Self {
        self.request.cached_content = Some(cache_name);
        self
    }

    pub fn with_chat_history(mut self, history: Vec<Content>) -> Self {
        self.request.contents = history;
        self
    }

    pub fn with_multiple_parts(mut self, parts: Vec<Part>) -> Self {
        if self.request.contents[0].parts[0].text == Some("Init message.".to_string()) {
            self.request.contents[0].parts = parts;
        } else {
            let content = Content {
                role: "user".to_string(),
                parts: parts,
            };
            self.request.contents.push(content);
        }
        self
    }

    pub fn with_tools(mut self, tools: Vec<serde_json::Value>) -> Self {
        self.request.tools = Some(tools);
        self
    }

    pub fn with_tool_config(mut self, tool_choice: serde_json::Value) -> Self {
        self.request.tool_config = Some(tool_choice);
        self
    }

    pub fn with_image(mut self, image: &str, mime_type: &str) -> Self {
        let content = Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: None,
                function_call: None,
                inline_data: Some(InlineData {
                    mime_type: mime_type.to_string(),
                    data: Some(image.to_string()),
                }),
                file_data: None,
            }],
        };
        self.request.contents.push(content);
        self
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EmbedGemini {
    pub base_url: String,
    pub model: String,
    pub request: EmbedRequest,
}

#[allow(dead_code)]
impl EmbedGemini {
    pub fn new(model: &str) -> Result<Self, GeminiError> {
        let api_key = Self::get_api_key()?;
        
        let base_url = format!(
            "{}/models/{}:embedContent?key={}",
            GEMINI_BASE_URL,
            model,
            api_key,
        );
        
        let request = EmbedRequest {
            model: model.to_string(),
            content: Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: Some("Init message.".to_string()),
                    function_call: None,
                    inline_data: None,
                    file_data: None,
                }],
            },
            output_dimensionality: None,
            task_type: TaskType::Unspecified,
            title: None,
        };
        
        Ok(Self {
            base_url: base_url,
            model: model.to_string(),
            request: request,
        })
    }

    pub async fn embed_content(
        mut self, 
        input_str: &str
    ) -> Result<EmbedResponse, GeminiError> {

        if self.request.content.parts[0].text == Some("Init message.".to_string()) {
            self.request.content.parts[0].text = Some(input_str.to_string());
        } else {
            let content = Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: Some(input_str.to_string()),
                    function_call: None,
                    inline_data: None,
                    file_data: None,
                }],
            };
            self.request.content = content;
        }

        let response: String = match request_embed(
            &self.base_url,
            self.request.clone(),
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(GeminiError::RequestEmbedError);
            }
        };

        let embed_response: EmbedResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(GeminiError::ResponseContentError);
            }
        };
        if let Some(error) = embed_response.error {
            println!("[ERROR] {:?}", error);
            return Err(GeminiError::ResponseContentError);
        } else {
            Ok(embed_response)
        }
    }

    pub fn with_output_dimensionality(mut self, output_dimensionality: i32) -> Self {
        self.request.output_dimensionality = Some(output_dimensionality);
        self
    }

    pub fn with_task_type(mut self, task_type: TaskType) -> Self {
        self.request.task_type = task_type;
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.request.title = Some(title.to_string());
        self
    }
}

impl GetApiKey for ChatGemini {}
impl GetApiKey for EmbedGemini {}