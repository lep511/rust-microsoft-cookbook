use reqwest::Client;
// use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::llmerror::ReplicateError;
use std::io::copy;
use std::fs::File;
use std::env;
use log::error;

pub static REPLICATE_BASE_URL: &str = " https://api.replicate.com/v1";

pub trait GetApiKey {
    fn get_api_key() -> Result<String, ReplicateError> {
        match env::var("REPLICATE_API_TOKEN") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                error!("Error[E001] REPLICATE_API_TOKEN not found in environment variables");
                Err(ReplicateError::ApiKeyNotFound)
            }
            Err(error) => {
                error!("Error[E002] {:?}", error);
                Err(ReplicateError::EnvError(error))
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReplicateModels {
    pub base_url: String,
    pub api_key: String,
    pub request: serde_json::Value,
    pub timeout: u64,
    pub client: Client,
}

#[allow(dead_code)]
impl ReplicateModels {
    pub fn new(model: &str) -> Result<Self, ReplicateError> {
        let api_key = Self::get_api_key()?;
        let request = serde_json::json!({});
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
    ) -> Result<serde_json::Value, ReplicateError> {
        
        self.request = input_data;

        // let _pretty_json = match serde_json::to_string_pretty(&self.request) {
        //     Ok(json) =>  println!("Pretty-printed JSON:\n{}", json),
        //     Err(error) => {
        //         error!("Error[E003] {:?}", error);
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
        //     Err(error) => {
        //         error!("Error[E004] {:?}", error);
        //     }
        // };

        if response.get("output").is_none() {
            error!("Error[E114] {:?}", response);
            match serde_json::to_string_pretty(&response) {
                Ok(response_form) => {
                    error!("Error[E114.1] {}", response_form);
                }
                Err(_) => {
                    error!("Error[E114.2] {:?}", response);
                }
            };
            return Err(ReplicateError::ResponseContentError);
        }
        
        Ok(response)
    }

    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub async fn get_file(
        self,
        file_url: &str,
    ) -> Result<String, ReplicateError> {
        // Send GET request and get the response bytes
        let response = self.client.get(file_url).send().await?;
        let bytes = response.bytes().await?;
        
        // Create a file to save the image
        let file_name = match file_url.split("/").last() {
            Some(name) => name,
            None => "output.jpg",
        };

        // Create a file to save the image
        let mut file = match File::create(file_name) {
            Ok(file) => file,
            Err(error) => {
                error!("Error[E117] {:?}", error);
                return Err(ReplicateError::FileCreateError);
            }
        };

        // Copy the bytes to the file
        match copy(&mut bytes.as_ref(), &mut file) {
            Ok(_) => {}
            Err(error) => {
                error!("Error[E118] {:?}", error);
                return Err(ReplicateError::FileCopyError);
            }
        };

        Ok(file_name.to_string())
    }

}

impl GetApiKey for ReplicateModels {}