use crate::llmerror::AssemblyError;
use log::{info, error};
use std::env;

/// Gets the ASSEMBLYAI_API_KEY from the environment variables
pub trait GetApiKey {
    fn get_api_key() -> Result<String, AssemblyError> {
        match env::var("ASSEMBLY_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                info!("ASSEMBLY_KEY not found in environment variables");
                Err(AssemblyError::ApiKeyNotFound)
            }
            Err(e) => {
                error!("Unable to read env ASSEMBLY_KEY {:?}", e);
                Err(AssemblyError::EnvError(e))
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
