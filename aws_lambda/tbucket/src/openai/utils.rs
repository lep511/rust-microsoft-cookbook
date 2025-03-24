use std::env;
use crate::openai::error::OpenAIError;
use log::{info, error};

/// Gets the API key from the environment variables
///
/// # Returns
/// * A string slice containing the API key
///
/// # Panics
/// * If the OPENAI_API_KEY environment variable is not set
///
/// # Examples
/// ```
/// let api_key = get_api_key();
/// println!("API key: {}", api_key);
/// ```
pub trait GetApiKey {
    fn get_api_key() -> Result<String, OpenAIError> {
        match env::var("OPENAI_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                info!("OPENAI_API_KEY not found in environment variables");
                Err(OpenAIError::ApiKeyNotFound)
            }
            Err(e) => {
                error!("Unable to read env OPENAI_API_KEY {:?}", e);
                Err(OpenAIError::EnvError(e))
            }
        }
    }
}

/// Prints the given request as a pretty-printed JSON string
///
/// # Arguments
/// * `request` - The request to be printed
///
pub fn print_pre(request: &impl serde::Serialize, active: bool) {
    if !active {
        println!();
    } else {
        match serde_json::to_string_pretty(request) {
            Ok(json) => println!("Pretty-printed JSON:\n{}", json),
            Err(e) => error!("Error {:?}", e)
        }
    }
}