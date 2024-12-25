use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::env;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum OpenAIChatError {
    #[error("OpenAI API key not found in environment variables")]
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
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_completion_tokens:  Option<u32>, // For O1 models
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
#[derive(Debug, Clone)]
pub struct ChatOpenAI {
    pub api_key: String,
    pub request: ChatRequest,
    pub timeout: u64,
    pub client: Client,
}

#[allow(dead_code)]
impl ChatOpenAI {
    const OPENAI_BASE_URL: &'static str = "https://api.openai.com/v1/chat/completions";

    pub fn new(model: &str) -> Result<Self, OpenAIChatError> {
        let api_key = match env::var("OPENAI_API_KEY") {
            Ok(key) => key,
            Err(env::VarError::NotPresent) => {
                return Err(OpenAIChatError::ApiKeyNotFound);
            }
            Err(e) => {
                return Err(OpenAIChatError::EnvError(e));
            }
        };

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
    ) -> Result<ChatResponse, OpenAIChatError> {
        
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

        // let pretty_json = match serde_json::to_string_pretty(&self.request) {
        //     Ok(json) =>  println!("Pretty-printed JSON:\n{}", json),
        //     Err(e) => {
        //         println!("[ERROR] {:?}", e);
        //     }
        // };

        let response = self
            .client
            .post(Self::OPENAI_BASE_URL)
            .timeout(Duration::from_secs(self.timeout))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&self.request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        
        let response = response.to_string();
        let chat_response: ChatResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(OpenAIChatError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            println!("[ERROR] {}", error.message);
            return Err(OpenAIChatError::ResponseContentError);
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
    content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Source>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Source {
    #[serde(rename = "type")]
    source_type: String,
    media_type: String,
    data: String,
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
    pub content: String,
    pub refusal: Option<String>,
    pub role: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Usage {
    pub completion_tokens: u32,
    pub completion_tokens_details: CompletionTokensDetails,
    pub prompt_tokens: u32,
    pub prompt_tokens_details: PromptTokensDetails,
    pub total_tokens: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CompletionTokensDetails {
    pub accepted_prediction_tokens: u32,
    pub audio_tokens: u32,
    pub reasoning_tokens: u32,
    pub rejected_prediction_tokens: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PromptTokensDetails {
    pub audio_tokens: u32,
    pub cached_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    pub param: Option<String>,
    #[serde(rename = "type")]
    pub error_type: String,
}