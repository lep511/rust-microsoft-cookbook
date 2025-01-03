use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::llmerror::ReplicateError;
use std::env;

pub static REPLICATE_BASE_URL: &str = " https://api.replicate.com/v1/models";

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct ChatRequest {
    pub input: serde_json::Value,
}

pub trait GetApiKey {
    fn get_api_key() -> Result<String, ReplicateError> {
        match env::var("REPLICATE_API_TOKEN") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                println!("[ERROR][E001] REPLICATE_API_TOKEN not found in environment variables");
                Err(ReplicateError::ApiKeyNotFound)
            }
            Err(e) => {
                println!("[ERROR][E002] {:?}", e);
                Err(ReplicateError::EnvError(e))
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReplicateModels {
    pub base_url: String,
    pub api_key: String,
    pub request: ChatRequest,
    pub timeout: u64,
    pub client: Client,
}

#[allow(dead_code)]
impl ReplicateModels {
    pub fn new(model: &str) -> Result<Self, ReplicateError> {
        let api_key = Self::get_api_key()?;

        let request = ChatRequest {
            input: serde_json::json!({}),
        };

        let base_url = format!("{}/{}", REPLICATE_BASE_URL, model);
        
        Ok(Self {
            base_url: base_url,
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
        input_data: serde_json::Value,
    ) -> Result<ReplicateResponse, ReplicateError> {
        
        self.request.input = input_data;

        // let _pretty_json = match serde_json::to_string_pretty(&self.request) {
        //     Ok(json) =>  println!("Pretty-printed JSON:\n{}", json),
        //     Err(e) => {
        //         println!("[ERROR][E003] {:?}", e);
        //     }
        // };

        let response = self
            .client
            .post(&self.base_url)
            .timeout(Duration::from_secs(self.timeout))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "wait")
            .json(&self.request)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        // let _pretty_json = match serde_json::to_string_pretty(&response) {
        //     Ok(json) =>  println!("Pretty-printed JSON:\n{}", json),
        //     Err(e) => {
        //         println!("[ERROR][E004] {:?}", e);
        //     }
        // };
        
        let response = response.to_string();

        // Check error in response
        let error_details: ErrorDetails = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(_) => {
                ErrorDetails {
                    detail: None,
                    invalid_fields: None,
                    status: None,
                    title: None,
                    found_error: Some(false),
                }
            }
        };

        if let Some(detail) = error_details.detail {
            println!("[ERROR][E105] {}", detail);
            return Err(ReplicateError::ResponseContentError);
        }

        let chat_response: ReplicateResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                println!("[ERROR][E005] {:?}", e);
                return Err(ReplicateError::ResponseContentError);
            }
        };

        if let Some(error) = chat_response.error {
            println!("[ERROR][E006] {}", error);
            return Err(ReplicateError::ResponseContentError);
        } else {
            Ok(chat_response)
        }
    }
}

impl GetApiKey for ReplicateModels {}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicateResponse {
    pub created: Option<String>,
    pub data_removed: Option<bool>,
    pub id: Option<String>,
    pub input : Option<serde_json::Value>,
    pub model: Option<String>,
    pub output: Option<Vec<String>>,
    pub status: Option<String>,
    pub urls: Option<serde_json::Value>,
    pub version: Option<String>,
    pub error: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub detail: Option<String>,
    pub invalid_fields: Option<serde_json::Value>,
    pub status: Option<i32>,
    pub title: Option<String>,
    pub found_error: Option<bool>,
}
