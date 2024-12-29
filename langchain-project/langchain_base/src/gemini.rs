use reqwest::Client;
use reqwest::{self, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{env, fs};
use serde_json::json;

pub static GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
pub static UPLOAD_BASE_URL: &str = "https://generativelanguage.googleapis.com/upload/v1beta";

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum GeminiChatError {
    #[error("Gemini API key not found in environment variables")]
    ApiKeyNotFound,
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Environment error: {0}")]
    EnvError(#[from] env::VarError),
    #[error("Failed to get response content")]
    ResponseContentError,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct ChatRequest {
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
    #[serde(rename = "systemInstruction")]
    pub system_instruction: Option<Content>,
    #[serde(rename = "generationConfig")]
    pub generation_config: Option<GenerationConfig>,
    #[serde(rename = "cachedContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_content: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functionCall", default)]
    pub function_call: Option<FunctionCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_data: Option<InlineData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<FileData>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InlineData {
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileData {
    pub mime_type: String,
    pub file_uri: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheRequest {
    pub model: String,
    pub contents: Vec<Content>,
    #[serde(rename = "systemInstruction")]
    pub system_instruction: Content,
    pub ttl: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  GenerationConfig {
    pub temperature: Option<f32>,
    #[serde(rename = "topK")]
    pub top_k: Option<u32>,
    #[serde(rename = "topP")]
    pub top_p: Option<f32>,
    #[serde(rename = "maxOutputTokens")]
    pub max_output_tokens: Option<u32>,
    #[serde(rename = "responseMimeType")]
    pub response_mime_type: Option<String>,
    #[serde(rename = "responseSchema")]
    pub response_schema: Option<serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatGemini {
    pub base_url: String,
    pub model: String,
    pub request: ChatRequest,
    pub timeout: u64,
    pub client: Client,
}

#[allow(dead_code)]
impl ChatGemini {
    fn get_api_key() -> Result<String, GeminiChatError> {
        match env::var("GEMINI_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                println!("[ERROR] GEMINI_API_KEY not found in environment variables");
                Err(GeminiChatError::ApiKeyNotFound)
            }
            Err(e) => {
                println!("[ERROR] {:?}", e);
                Err(GeminiChatError::EnvError(e))
            }
        }
    }

    pub fn new(model: &str) -> Result<Self, GeminiChatError> {
        // let api_key = match env::var("GEMINI_API_KEY") {
        //     Ok(key) => key,
        //     Err(env::VarError::NotPresent) => {
        //         println!("[ERROR] GEMINI_API_KEY not found in environment variables");
        //         return Err(GeminiChatError::ApiKeyNotFound);
        //     }
        //     Err(e) => {
        //         println!("[ERROR] {:?}", e);
        //         return Err(GeminiChatError::EnvError(e));
        //     }
        // };
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
            system_instruction: None,
            cached_content: None,
            generation_config: Some(GenerationConfig {
                temperature: Some(0.9),
                top_k: Some(40),
                top_p: Some(0.95),
                max_output_tokens: Some(2048),
                response_mime_type: Some("text/plain".to_string()),
                response_schema: None,
            }),
        };
        
        Ok(Self {
            base_url: base_url,
            model: model.to_string(),
            request: request,
            timeout: 15 * 60, // default: 15 minutes
            client: Client::builder()
                .use_rustls_tls()
                .build()?,
        })
    }

    pub async fn invoke(mut self, prompt: &str) -> Result<ChatResponse, GeminiChatError> {

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

        // let _pretty_json = match serde_json::to_string_pretty(&self.request) {
        //     Ok(json) =>  println!("Pretty-printed JSON:\n{}", json),
        //     Err(e) => {
        //         println!("[ERROR] {:?}", e);
        //     }
        // };

        let response = self
            .client
            .post(self.base_url)
            .timeout(Duration::from_secs(self.timeout))
            .header("Content-Type", "application/json")
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
        let chat_response: ChatResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(GeminiChatError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            println!("[ERROR] {:?}", error);
            return Err(GeminiChatError::ResponseContentError);
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

    pub async fn media_upload(self, img_path: &str, mime_type: &str) -> Result<String, Box<dyn std::error::Error>> {
        let api_key = Self::get_api_key()?;
        let upload_url = format!(
            "{}/files?key={}",
            UPLOAD_BASE_URL,
            api_key
        );
        
        let display_name = "TEXT";
        let num_bytes = fs::metadata(&img_path)?.len();
        let num_bytes = num_bytes.to_string();

        let mut headers = HeaderMap::new();
        headers.insert("X-Goog-Upload-Protocol", HeaderValue::from_static("resumable"));
        headers.insert("X-Goog-Upload-Command", HeaderValue::from_static("start"));
        headers.insert("X-Goog-Upload-Header-Content-Length", HeaderValue::from_str(&num_bytes)?);
        headers.insert("X-Goog-Upload-Header-Content-Type", HeaderValue::from_str(&mime_type)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let initial_resp = self
            .client
            .post(upload_url)
            .headers(headers)
            .json(&json!({
                "file": {
                    "display_name": display_name
                }
            }))
            .send()
            .await?;

        // Get upload URL from response headers
        let upload_url = initial_resp
            .headers()
            .get("x-goog-upload-url")
            .ok_or("Missing upload URL")?
            .to_str()?;

        // Upload file content
        let file_content = fs::read(&img_path)?;
        let mut upload_headers = HeaderMap::new();
        upload_headers.insert("Content-Length", HeaderValue::from_str(&num_bytes)?);
        upload_headers.insert("X-Goog-Upload-Offset", HeaderValue::from_static("0"));
        upload_headers.insert("X-Goog-Upload-Command", HeaderValue::from_static("upload, finalize")); 

        let upload_resp: serde_json::Value = self
            .client
            .post(upload_url)
            .headers(upload_headers)
            .body(file_content)
            .send()
            .await?
            .json()
            .await?;

        let file_uri = upload_resp["file"]["uri"]
            .as_str()
            .ok_or("Missing file URI")?
            .trim_matches('"')
            .to_string();

        Ok(file_uri)
    }

    pub async fn cache_upload(self, data: String, mime_type: &str, instruction: &str) -> Result<String, Box<dyn std::error::Error>> {
        let api_key = Self::get_api_key()?;
        let url_cache = format!(
            "{}/cachedContents?key={}", 
            GEMINI_BASE_URL,
            api_key
        );

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        let model_name = format!("models/{}", self.model);

        let part_system_instruction = vec![Part {
            text: Some(instruction.to_string()),
            function_call: None,
            inline_data: None,
            file_data: None,
        }];

        let system_instruction = Content {
            role: "user".to_string(),
            parts: part_system_instruction,
        };

        let cache_request = CacheRequest {
            model: model_name,
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: None,
                    function_call: None,
                    inline_data: Some(InlineData {
                        mime_type: mime_type.to_string(),
                        data: Some(data),
                    }),
                    file_data: None,
                }],
            }],
            system_instruction: system_instruction,
            ttl: "300s".to_string(),
        };

        let cache_resp: serde_json::Value = self
            .client
            .post(url_cache)
            .headers(headers)
            .json(&cache_request)
            .send()
            .await?
            .json()
            .await?;
    
        let cache_name = cache_resp["name"]
            .as_str()
            .ok_or("Missing cache name")?
            .trim_matches('"')
            .to_string();

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

    pub fn with_file_url(mut self, image_url: String, mime_type: &str) -> Self {
        let content = Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: None,
                function_call: None,
                inline_data: None,
                file_data: Some(FileData {
                    mime_type: mime_type.to_string(),
                    file_uri: image_url,
                }),
            }]
        };
        self.request.contents.push(content);
        self
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub candidates: Option<Vec<Candidate>>,
    pub model_version: Option<String>,
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: Option<UsageMetadata>,
    pub chat_history: Option<Vec<Content>>,
    pub error: Option<ErrorDetails>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    #[serde(rename = "avgLogprobs")]
    pub avg_logprobs: Option<f64>,
    pub content: Option<Content>,
    #[serde(rename = "finishReason")]
    pub finish_reason: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub args: String,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct UsageMetadata {
    #[serde(rename = "candidatesTokenCount")]
    pub candidates_token_count: i32,
    #[serde(rename = "promptTokenCount")]
    pub rompt_token_count: i32,
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: i32,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: Option<i32>,
    pub message: Option<String>,
    pub status: Option<String>,
    pub details: Option<Vec<ErrorDetail>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "@type")]
    pub type_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub service: String,
}
