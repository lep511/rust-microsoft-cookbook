use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::llmerror::OpenAIError;
use std::env;

pub static OPENAI_BASE_URL: &str = "https://api.openai.com/v1/chat/completions";
pub static OPENAI_EMBED_URL: &str = "https://api.openai.com/v1/embeddings";

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_completion_tokens:  Option<u32>, // For O1 models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(rename = "n")]
    pub n_completion: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub response_type: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedRequest {
    pub model: String,
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
}

pub trait GetApiKey {
    fn get_api_key() -> Result<String, OpenAIError> {
        match env::var("OPENAI_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                println!("[ERROR] OPENAI_API_KEY not found in environment variables");
                Err(OpenAIError::ApiKeyNotFound)
            }
            Err(e) => {
                println!("[ERROR] {:?}", e);
                Err(OpenAIError::EnvError(e))
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatOpenAI {
    pub api_key: String,
    pub request: ChatRequest,
    pub timeout: u64,
    pub client: Client,
}

#[allow(dead_code)]
impl ChatOpenAI {
    pub fn new(model: &str) -> Result<Self, OpenAIError> {
        let api_key = Self::get_api_key()?;

        let dev_prompt = "You are a helpful assistant.".to_string();

        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(dev_prompt),
            source: None,
        }];

        let messages = vec![Message {
            role: Role::Developer,
            content: content.clone(),
            recipient: None,
            end_turn: None,
        }];

        let response_format = ResponseFormat {
            response_type: "text".to_string(),
        };

        let request = ChatRequest {
            model: model.to_string(),
            messages: messages.clone(),
            temperature: Some(0.9),
            tools: None,
            tool_choice: None,
            max_completion_tokens: Some(1024),
            response_format: Some(response_format),
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
            client: Client::builder()
                .use_rustls_tls()
                .build()?,
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
        self.request.messages.push(Message {
            role: Role::User,
            content: content.clone(),
            recipient: None,
            end_turn: None,
        });

        // let _pretty_json = match serde_json::to_string_pretty(&self.request) {
        //     Ok(json) =>  println!("Pretty-printed JSON:\n{}", json),
        //     Err(e) => {
        //         println!("[ERROR] {:?}", e);
        //     }
        // };

        let response = self
            .client
            .post(OPENAI_BASE_URL)
            .timeout(Duration::from_secs(self.timeout))
            .header("Authorization", format!("Bearer {}", self.api_key))
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
                chat_history: Some(self.request.messages),
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
        self.request.messages[0].content = content;
        self
    }

    pub fn with_assistant_response(mut self,  assistant_response: &str) -> Self {
        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(assistant_response.to_string()),
            source: None,
        }];
        self.request.messages.push(Message {
            role: Role::Assistant,
            content: content.clone(),
            recipient: None,
            end_turn: None,
        });
        self
    }

    pub fn with_chat_history(mut self, history: Vec<Message>) -> Self {
        self.request.messages = history;
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EmbedOpenAI {
    pub model: String,
    pub request: EmbedRequest,
    pub timeout: u64,
    pub api_key: String,
    pub client: Client,
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
            client: Client::builder()
                .use_rustls_tls()
                .build()?,
        })
    }

    pub async fn embed_content(mut self, input_str: &str) -> Result<EmbedResponse, OpenAIError> {
        self.request.input = input_str.to_string();
        
        let response = self
            .client
            .post(OPENAI_EMBED_URL)
            .timeout(Duration::from_secs(self.timeout))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&self.request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let response = response.to_string();
        let embed_response: EmbedResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(OpenAIError::ResponseContentError);
            }
        };
        if let Some(error) = embed_response.error {
            println!("[ERROR] {}", error.message);
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

impl GetApiKey for ChatOpenAI {}
impl GetApiKey for EmbedOpenAI {}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedResponse {
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub object: String,
    pub usage: Usage,
    pub error: Option<ErrorDetails>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingData {
    pub embedding: Vec<f32>,
    pub index: i32,
    pub object: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Platform,
    Developer,
    User,
    Assistant,
    Tool,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: Vec<InputContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_turn: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputContent {
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Source {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Option<Vec<Choice>>,
    pub created: Option<u64>,
    pub id: Option<String>,
    pub model: Option<String>,
    pub object: Option<String>,
    pub system_fingerprint: Option<String>,
    pub usage: Option<Usage>,
    pub chat_history: Option<Vec<Message>>,
    pub error: Option<ErrorDetails>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub finish_reason: String,
    pub index: u32,
    pub logprobs: Option<serde_json::Value>,
    pub message: ChatMessage,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    pub content: Option<String>,
    pub refusal: Option<String>,
    pub role: String,
    pub tool_calls: Option<Vec<serde_json::Value>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub completion_tokens: Option<u32>,
    pub completion_tokens_details: Option<CompletionTokensDetails>,
    pub prompt_tokens: Option<u32>,
    pub prompt_tokens_details: Option<PromptTokensDetails>,
    pub total_tokens: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionTokensDetails {
    pub accepted_prediction_tokens: u32,
    pub audio_tokens: u32,
    pub reasoning_tokens: u32,
    pub rejected_prediction_tokens: u32,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PromptTokensDetails {
    pub audio_tokens: u32,
    pub cached_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
}
