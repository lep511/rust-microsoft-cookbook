use std::env;
use crate::llmerror::CompatibleChatError;

/// Gets the API key from the environment variables
///
/// # Returns
/// * A string slice containing the API key
///
/// # Panics
/// * If the COMPATIBLE_API_KEY environment variable is not set
///
/// # Examples
/// ```
/// let api_key = get_api_key();
/// println!("API key: {}", api_key);
/// ```
pub trait GetApiKey {
    fn get_api_key() -> Result<String, CompatibleChatError> {
        match env::var("COMPATIBLE_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                println!("[ERROR] COMPATIBLE_API_KEY not found in environment variables");
                Err(CompatibleChatError::ApiKeyNotFound)
            }
            Err(e) => {
                println!("[ERROR] {:?}", e);
                Err(CompatibleChatError::EnvError(e))
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