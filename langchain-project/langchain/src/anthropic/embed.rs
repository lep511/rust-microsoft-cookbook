use crate::anthropic::utils::GetApiKeyVoyage;
use crate::llmerror::AnthropicError;

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
        let init_msg = InputEmbed::String("".to_string());
        let request = EmbedRequest {
            model: model.to_string(),
            input: init_msg,
            output_dimension: None,
            output_dtype: None,
            encoding_format: None,
        };

        Ok(Self {
            model: model.to_string(),
            request: request,
            api_key: api_key,
        })
    }

    pub async fn embed_content(mut self, input: InputEmbed) -> Result<EmbedResponse, AnthropicError> {
        self.request.input = input;
        
        // let _pretty_json = match serde_json::to_string_pretty(&self.request) {
        //     Ok(json) =>  println!("Pretty-printed JSON:\n{}", json),
        //     Err(e) => {
        //         println!("[ERROR] {:?}", e);
        //     }
        // };

        let response = self
            .client
            .post(ANTHROPIC_EMBED_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .json(&self.request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        
        // let _pretty_json = match serde_json::to_string_pretty(&response) {
        //     Ok(json) =>  println!("Pretty-printed JSON:\n{}", json),
        //     Err(e) => {
        //         println!("[ERROR] {:?}", e);
        //     }
        // };

        let response = response.to_string();
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
    pub request: EmbedMultiRequest,
    pub api_key: String,
}

#[allow(dead_code)]
impl EmbedMultiVoyage {
    pub fn new(model: &str) -> Result<Self, AnthropicError> {
        let api_key = Self::get_api_key()?;
        let init_msg = InputContent {
            content_type: "text".to_string(),
            text: Some("".to_string()),
            source: None,
            image_url: None,
            image_base64: None,
        };

        let request = EmbedMultiRequest {
            model: model.to_string(),
            inputs: vec![EmbedContent {
                content: vec![init_msg],
            }],
            output_encoding: None,
            input_type: None,
            truncation: None,
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
        self.request.inputs[0].content[0].text = Some(input_str.to_string());
        
        // let _pretty_json = match serde_json::to_string_pretty(&self.request) {
        //     Ok(json) =>  println!("Pretty-printed JSON:\n{}", json),
        //     Err(e) => {
        //         println!("[ERROR] {:?}", e);
        //     }
        // };

        let response = self
            .client
            .post(ANTHROPIC_EMBEDMUL_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .json(&self.request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        
        // let _pretty_json = match serde_json::to_string_pretty(&response) {
        //     Ok(json) =>  println!("Pretty-printed JSON:\n{}", json),
        //     Err(e) => {
        //         println!("[ERROR] {:?}", e);
        //     }
        // };

        let response = response.to_string();
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
        let content = InputContent {
            content_type: "image_url".to_string(),
            text: None,
            source: None,
            image_url: Some(image_url.to_string()),
            image_base64: None,
        };
        self.request.inputs[0].content.push(content);
        
        self
    }

    pub fn with_image_base64(
        mut self, 
        image_base64: &str, 
        media_type: &str
    ) -> Self {
        let media_types_supported = [
            "image/png", 
            "image/jpeg",
            "image/jpg",
            "image/gif", 
            "image/webp", 
            "image/gif"
        ];

        if !media_types_supported.contains(&media_type) {
            println!(
                "[ERROR] Unsupported media type: {}. Supported: image/png, \
                image/jpeg, image/webp, and image/gif", 
                media_type
            );
            return self;
        }

        let format_base64 = format!("data:{};base64,{}", media_type, image_base64);
        let content = InputContent {
            content_type: "image_base64".to_string(),
            text: None,
            source: None,
            image_url: None,
            image_base64: Some(format_base64.to_string()),
        };
        self.request.inputs[0].content.push(content);

        self
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EmbedRankVoyage {
    pub model: String,
    pub request: EmbedRankRequest,
    pub api_key: String,
    pub client: Client,
}

#[allow(dead_code)]
impl EmbedRankVoyage {
    pub fn new(model: &str) -> Result<Self, AnthropicError> {
        let api_key = Self::get_api_key()?;
        
        let request = EmbedRankRequest {
            model: model.to_string(),
            query: "".to_string(),
            documents: vec![],
            top_k: None,
            truncation: None,
        };

        Ok(Self {
            model: model.to_string(),
            request: request,
            api_key: api_key,
            client: Client::builder()
                .use_rustls_tls()
                .build()?,
        })
    }

    pub async fn embed_content(
        mut self, 
        input_str: &str
    ) -> Result<EmbedResponse, AnthropicError> {
        self.request.query = input_str.to_string();
        
        // let _pretty_json = match serde_json::to_string_pretty(&self.request) {
        //     Ok(json) =>  println!("Pretty-printed JSON:\n{}", json),
        //     Err(e) => {
        //         println!("[ERROR] {:?}", e);
        //     }
        // };

        let response = self
            .client
            .post(ANTHROPIC_EMBEDRANK_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .json(&self.request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        
        // let _pretty_json = match serde_json::to_string_pretty(&response) {
        //     Ok(json) =>  println!("Pretty-printed JSON:\n{}", json),
        //     Err(e) => {
        //         println!("[ERROR] {:?}", e);
        //     }
        // };

        let response = response.to_string();
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
        self.request.documents = documents;
        self
    }
}

impl GetApiKeyVoyage for EmbedVoyage {}
impl GetApiKeyVoyage for EmbedMultiVoyage {}
impl GetApiKeyVoyage for EmbedRankVoyage {}