use crate::llmerror::GeminiError;
use crate::gemini::utils::GetApiKey;
use crate::gemini::libs::{
    Content, Part, EmbedResponse,EmbedRequest, TaskType
};
use crate::gemini::requests::request_embed;
use std::time::Duration;
use log::error;

pub static GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EmbedGemini {
    pub base_url: String,
    pub model: String,
    pub request: EmbedRequest,
    pub max_retries: u32,
    pub timeout: Duration,
}

#[allow(dead_code)]
impl EmbedGemini {
    pub fn new(model: &str) -> Self {
        let api_key: String = match Self::get_api_key() {
            Ok(api_key) => api_key,
            Err(_) => "not_key".to_string()
        };
        
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
                    function_response: None,
                    inline_data: None,
                    file_data: None,
                }],
            },
            output_dimensionality: None,
            task_type: TaskType::Unspecified,
            title: None,
        };
        
        Self {
            base_url: base_url,
            model: model.to_string(),
            request: request,
            max_retries: 0,
            timeout: Duration::from_secs(300), // default: 5 minutes
        }
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
                    function_response: None,
                    inline_data: None,
                    file_data: None,
                }],
            };
            self.request.content = content;
        }

        let response: String = match request_embed(
            &self.base_url,
            self.request.clone(),
            self.max_retries,
            self.timeout,
        ).await {
            Ok(response) => response,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(GeminiError::RequestEmbedError);
            }
        };

        let embed_response: EmbedResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(GeminiError::ResponseContentError);
            }
        };
        if let Some(error) = embed_response.error {
            error!("Error {:?}", error);
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

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }
    
    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = Duration::from_secs(timeout);
        self
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.base_url = format!(
            "{}/models/{}:embedContent?key={}",
            GEMINI_BASE_URL,
            self.model,
            api_key,
        );

        self
    }
}

impl GetApiKey for EmbedGemini {}