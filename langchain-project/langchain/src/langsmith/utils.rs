use crate::llmerror::LangsmithError;
use std::env;

/// Gets the LANGSMITH_API_KEY from the environment variables
pub trait GetApiKey {
    fn get_api_key() -> Result<String, LangsmithError> {
        match env::var("LANGSMITH_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                println!("[ERROR] LANGSMITH_API_KEY not found in environment variables");
                Err(LangsmithError::ApiKeyNotFound)
            }
            Err(e) => {
                println!("[ERROR] {:?}", e);
                Err(LangsmithError::EnvError(e))
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