use crate::llmerror::GeminiError;
use crate::gemini::utils::{GetApiKey, TaskType};
use crate::gemini::libs::{
    Content, Part, EmbedResponse,EmbedRequest,
};
use crate::gemini::requests::request_embed;

pub static GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EmbedGemini {
    pub base_url: String,
    pub model: String,
    pub request: EmbedRequest,
    pub retry: u32,
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
                    function_response: None,
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
            retry: 0,
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
            self.retry,
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

    pub fn with_retry(mut self, retry: u32) -> Self {
        self.retry = retry;
        self
    }
}

impl GetApiKey for EmbedGemini {}