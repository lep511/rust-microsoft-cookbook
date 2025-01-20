use crate::llmerror::AnthropicError;
use std::env;

/// Gets the ANTHROPIC_API_KEY from the environment variables
pub trait GetApiKey {
    fn get_api_key() -> Result<String, AnthropicError> {
        match env::var("ANTHROPIC_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                println!("[ERROR] ANTHROPIC_API_KEY not found in environment variables");
                Err(AnthropicError::ApiKeyNotFound)
            }
            Err(e) => {
                println!("[ERROR] {:?}", e);
                Err(AnthropicError::EnvError(e))
            }
        }
    }
}

/// Gets the VOYAGE_API_KEY from the environment variables
pub trait GetApiKeyVoyage {
    fn get_api_key() -> Result<String, AnthropicError> {
        match env::var("VOYAGE_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                println!("[ERROR] VOYAGE_API_KEY not found in environment variables");
                Err(AnthropicError::ApiKeyNotFound)
            }
            Err(e) => {
                println!("[ERROR] {:?}", e);
                Err(AnthropicError::EnvError(e))
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
            Err(e) => println!("[ERROR] {:?}", e)
        }
    }
}