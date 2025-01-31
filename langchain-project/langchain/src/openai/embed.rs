use crate::openai::requests::request_embed;
use crate::openai::libs::{EmbedRequest, EmbedResponse};
use crate::openai::utils::GetApiKey;
use crate::llmerror::OpenAIError;
use log::error;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EmbedOpenAI {
    pub model: String,
    pub request: EmbedRequest,
    pub timeout: u64,
    pub api_key: String,
}

#[allow(dead_code)]
impl EmbedOpenAI {
    pub fn new(model: &str) -> Result<Self, OpenAIError> {
        let api_key = Self::get_api_key()?;
        let request = EmbedRequest {
            model: model.to_string(),
            input: "Init message.".to_string(),
            dimensions: None,
        };

        Ok(Self {
            model: model.to_string(),
            request: request,
            timeout: 15 * 60, // default: 15 minutes
            api_key: api_key,
        })
    }

    pub async fn embed_content(mut self, input_str: &str) -> Result<EmbedResponse, OpenAIError> {
        self.request.input = input_str.to_string();
        
        let response: String = match request_embed(
            &self.request,
            &self.api_key,
        ).await {
            Ok(response) => response,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(OpenAIError::ResponseContentError);
            }
        };

        let embed_response: EmbedResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Error {:?}", e);
                return Err(OpenAIError::ResponseContentError);
            }
        };
        if let Some(error) = embed_response.error {
            error!("Error {}", error.message);
            return Err(OpenAIError::ResponseContentError);
        } else {
            Ok(embed_response)
        }    
    }

    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_dimensions(mut self, dimensions: u32) -> Self {
        // Only supported in text-embedding-3 and later models
        self.request.dimensions = Some(dimensions);
        self
    }
}

impl GetApiKey for EmbedOpenAI {}