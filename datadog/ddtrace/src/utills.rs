use crate::error::DatadogError;

/// Gets the DD-API-KEY from the environment variables
pub trait GetApiKey {
    fn get_api_key() -> Result<String, DatadogError> {
        match env::var("DD-API-KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                info!("DD-API-KEY not found in environment variables");
                Err(DatadogError::ApiKeyNotFound)
            }
            Err(e) => {
                error!("Unable to read env DD-API-KEY {:?}", e);
                Err(DatadogError::EnvError(e))
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