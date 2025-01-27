use crate::anthropic::MIME_TYPE_SUPPORTED;
use crate::anthropic::requests::request_embed;
use crate::anthropic::utils::GetApiKeyVoyage;
use crate::llmerror::AnthropicError;
use crate::anthropic::libs::{
    EmbedRequest, Content, InputEmbed, EmbedContent,
    EmbedResponse, AnthropicEmbedEndpoint
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EmbedVoyage {
    pub model: String,
    pub request: EmbedRequest,
    pub api_key: String,
}

#[allow(dead_code)]
impl EmbedVoyage {
    pub fn new(model: &str) -> Result<Self, AnthropicError> {
        let api_key = Self::get_api_key()?;
        let request = EmbedRequest {
            model: model.to_string(),
            input: None,
            output_dimension: None,
            output_dtype: None,
            encoding_format: None,
            output_encoding: None,
            input_type: None,
            truncation: None,
            query: None,
            documents: None,
            top_k: None,
            inputs: None,
        };

        Ok(Self {
            model: model.to_string(),
            request: request,
            api_key: api_key,
        })
    }

    pub async fn embed_content(
        mut self, 
        input: InputEmbed
    ) -> Result<EmbedResponse, AnthropicError> {
        self.request.input = Some(input);

        let endpoint = AnthropicEmbedEndpoint::Embed;

        let response: String = match request_embed(
            &self.request,
            &self.api_key,
            endpoint
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(AnthropicError::ResponseContentError);
            }
        };
               
        let embed_response: EmbedResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(AnthropicError::ResponseContentError);
            }
        };
        if let Some(detail) = embed_response.detail {
            println!("[ERROR] {}", detail);
            return Err(AnthropicError::ResponseContentError);
        } else {
            Ok(embed_response)
        }
    }

    pub fn with_dimensions(mut self, dimensions: u32) -> Self {
        // Only supported in text-embedding-3 and later models
        self.request.output_dimension = Some(dimensions);
        self
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EmbedMultiVoyage {
    pub model: String,
    pub request: EmbedRequest,
    pub api_key: String,
}

#[allow(dead_code)]
impl EmbedMultiVoyage {
    pub fn new(model: &str) -> Result<Self, AnthropicError> {
        let api_key = Self::get_api_key()?;

        let request = EmbedRequest {
            model: model.to_string(),
            inputs: None,
            output_encoding: None,
            input_type: None,
            truncation: None,
            query: None,
            documents: None,
            top_k: None,
            output_dimension: None,
            output_dtype: None,
            encoding_format: None,
            input: None,
        };

        Ok(Self {
            model: model.to_string(),
            request: request,
            api_key: api_key,
        })
    }

    pub async fn embed_content(
        mut self, 
        input_str: &str
    ) -> Result<EmbedResponse, AnthropicError> {
        let content = Content {
            content_type: "text".to_string(),
            text: Some(input_str.to_string()),
            source: None,
            image_url: None,
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        };

        let embed_content = EmbedContent {
            content: vec![content],
        };

        if let Some(inputs) = &mut self.request.inputs {
            inputs.push(embed_content);
        } else {
            self.request.inputs = Some(vec![embed_content]);
        }
        
        let endpoint = AnthropicEmbedEndpoint::MultimodalEmbed;

        let response: String = match request_embed(
            &self.request,
            &self.api_key,
            endpoint,
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(AnthropicError::ResponseContentError);
            }
        };

        let embed_response: EmbedResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(AnthropicError::ResponseContentError);
            }
        };
        if let Some(detail) = embed_response.detail {
            println!("[ERROR] {}", detail);
            return Err(AnthropicError::ResponseContentError);
        } else {
            Ok(embed_response)
        }
    }

    pub fn with_image_url(mut self, image_url: &str) -> Self {
        let content = Content {
            content_type: "image_url".to_string(),
            text: None,
            source: None,
            image_url: Some(image_url.to_string()),
            image_base64: None,
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        };

        let embed_content = EmbedContent {
            content: vec![content],
        };

        if let Some(inputs) = &mut self.request.inputs {
            inputs.push(embed_content);
        } else {
            self.request.inputs = Some(vec![embed_content]);
        }
       
        self
    }

    pub fn with_image(
        mut self, 
        image_base64: &str, 
        media_type: &str
    ) -> Self {

        if !MIME_TYPE_SUPPORTED.contains(&media_type) {
            println!(
                "[ERROR] Unsupported media type: {}. Supported: {}", 
                media_type,
                MIME_TYPE_SUPPORTED.join(", ")
            );
            return self;
        }

        let format_base64 = format!("data:{};base64,{}", media_type, image_base64);
        let content = Content {
            content_type: "image_base64".to_string(),
            text: None,
            source: None,
            image_url: None,
            image_base64: Some(format_base64.to_string()),
            id: None,
            name: None,
            input: None,
            content: None,
            tool_use_id: None,
        };

        let embed_content = EmbedContent {
            content: vec![content],
        };

        if let Some(inputs) = &mut self.request.inputs {
            inputs.push(embed_content);
        } else {
            self.request.inputs = Some(vec![embed_content]);
        }

        self
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EmbedRankVoyage {
    pub model: String,
    pub request: EmbedRequest,
    pub api_key: String,
}

#[allow(dead_code)]
impl EmbedRankVoyage {
    pub fn new(model: &str) -> Result<Self, AnthropicError> {
        let api_key = Self::get_api_key()?;
        
        let request = EmbedRequest {
            model: model.to_string(),
            query: Some("".to_string()),
            documents: Some(vec![]),
            top_k: None,
            truncation: None,
            output_encoding: None,
            input_type: None,
            output_dtype: None,
            encoding_format: None,
            output_dimension: None,
            input: None,
            inputs: None,
        };

        Ok(Self {
            model: model.to_string(),
            request: request,
            api_key: api_key,
        })
    }

    pub async fn embed_content(
        mut self, 
        input_str: &str
    ) -> Result<EmbedResponse, AnthropicError> {
        self.request.query = Some(input_str.to_string());

        let endpoint = AnthropicEmbedEndpoint::Rerank;
        
        let response: String = match request_embed(
            &self.request,
            &self.api_key,
            endpoint,
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(AnthropicError::ResponseContentError);
            }
        };
        
        let embed_response: EmbedResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(AnthropicError::ResponseContentError);
            }
        };
        if let Some(detail) = embed_response.detail {
            println!("[ERROR] {}", detail);
            return Err(AnthropicError::ResponseContentError);
        } else {
            Ok(embed_response)
        }
    }

    pub fn with_documents(mut self, documents: Vec<String>) -> Self {
        self.request.documents = Some(documents);
        self
    }
}

impl GetApiKeyVoyage for EmbedVoyage {}
impl GetApiKeyVoyage for EmbedMultiVoyage {}
impl GetApiKeyVoyage for EmbedRankVoyage {}