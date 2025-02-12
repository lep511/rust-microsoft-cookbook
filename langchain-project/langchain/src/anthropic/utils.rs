use crate::llmerror::AnthropicError;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::fs::File;
use std::io::Read;
use log::{info, error};
use std::env;

/// Gets the ANTHROPIC_API_KEY from the environment variables
pub trait GetApiKey {
    fn get_api_key() -> Result<String, AnthropicError> {
        match env::var("ANTHROPIC_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                info!("ANTHROPIC_API_KEY not found in environment variables");
                Err(AnthropicError::ApiKeyNotFound)
            }
            Err(e) => {
                error!("Unable to read env ANTHROPIC_API_KEY {:?}", e);
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
                info!("VOYAGE_API_KEY not found in environment variables");
                Err(AnthropicError::VoyageApiKeyNotFound)
            }
            Err(e) => {
                error!("Unable to read env VOYAGE_API_KEY {:?}", e);
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
            Err(e) => error!("Error {:?}", e)
        }
    }
}

pub fn read_file_data(file_path: &str) -> Result<String, AnthropicError> {
    // Attempt to open the file at file_path
    let mut file = File::open(file_path)
        .map_err(|e| AnthropicError::FileError(e.to_string()))?;

    // Create a new empty, mutable vector that can grow dynamically to store the file contents
    let mut buffer = Vec::new();
    
    // Read the entire contents of the file into the buffer
    // read_to_end will read from the current position until EOF
    Read::read_to_end(&mut file, &mut buffer)
        .map_err(|e| AnthropicError::FileError(e.to_string()))?;

    // Encode the contents of the buffer into a base64 string using the STANDARD base64 alphabet
    let base64_encoded = STANDARD.encode(&buffer);
    
    Ok(base64_encoded)
}

