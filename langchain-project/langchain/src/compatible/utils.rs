use std::env;
use log::{info, error};
use crate::llmerror::CompatibleChatError;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::fs::File;
use std::io::Read;

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
                info!("COMPATIBLE_API_KEY not found in environment variables");
                Err(CompatibleChatError::ApiKeyNotFound)
            }
            Err(e) => {
                error!("Unable to read env COMPATIBLE_API_KEY {:?}", e);
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
            Err(e) => error!("Error {:?}", e)
        }
    }
}

pub fn read_file_data(file_path: &str) -> Result<String, CompatibleChatError> {
    // Attempt to open the file at file_path
    let mut file = File::open(file_path)
        .map_err(|e| CompatibleChatError::FileError(e.to_string()))?;

    // Create a new empty, mutable vector that can grow dynamically to store the file contents
    let mut buffer = Vec::new();
    
    // Read the entire contents of the file into the buffer
    // read_to_end will read from the current position until EOF
    Read::read_to_end(&mut file, &mut buffer)
        .map_err(|e| CompatibleChatError::FileError(e.to_string()))?;

    // Encode the contents of the buffer into a base64 string using the STANDARD base64 alphabet
    let base64_encoded = STANDARD.encode(&buffer);
    
    Ok(base64_encoded)
}