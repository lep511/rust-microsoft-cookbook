use std::env;
use super::llmerror::GeminiError;

/// Gets the API key from the environment variables
///
/// # Returns
/// * A string slice containing the API key
///
/// # Panics
/// * If the GEMINI_API_KEY environment variable is not set
///
/// # Examples
/// ```
/// let api_key = get_api_key();
/// println!("API key: {}", api_key);
/// ```
pub trait GetApiKey {
    fn get_api_key() -> Result<String, GeminiError> {
        match env::var("GEMINI_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                println!("[ERROR] GEMINI_API_KEY not found in environment variables");
                Err(GeminiError::ApiKeyNotFound)
            }
            Err(e) => {
                println!("[ERROR] {:?}", e);
                Err(GeminiError::EnvError(e))
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

/// Gets the MIME type for a given file extension
/// 
/// # Arguments
/// * `extension` - The file extension (without the dot)
/// 
/// # Returns
/// * A string slice containing the MIME type. Returns "application/octet-stream" for unknown extensions
/// 
/// # Examples
/// ```
/// let mime = get_mime_type("jpg");
/// assert_eq!(mime, "image/jpeg");
/// ```
pub fn get_mime_type(extension: &str) -> &'static str {
    let mime = match extension {
        "jpg" | "jpeg" => "image/jpeg",
        "png"   =>  "image/png",
        "webp"  =>  "image/webp",
        "gif"   =>  "image/gif",
        "mp4"   =>  "video/mp4",
        "flv"   =>  "video/x-flv",
        "mov"   =>  "video/quicktime",
        "mpg"   =>  "video/mpeg",
        "mpeg"  =>  "video/mpeg",
        "mpegs" =>  "video/mpeg",
        "3gpp"  =>  "video/3gpp",
        "webm"  =>  "video/webm",
        "wmv"   =>  "video/x-ms-wmv",
        "pdf"   =>  "application/pdf",
        "doc"   =>  "application/msword",
        "docx"  =>  "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "rtf"   =>  "application/rtf",
        "dot"   =>  "application/msword",
        "dotx"  =>  "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
        "txt"   =>  "text/plain",
        "csv"   =>  "text/csv",
        "tsv"   =>  "text/tab-separated-values",
        "xls"   =>  "application/vnd.ms-excel",
        "xlsx"  =>  "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "mp3"   =>  "audio/mpeg",
        "aac"   =>  "audio/aac",
        "mpa"   =>  "audio/mpeg",
        "flac"  =>  "audio/flac",
        "wav"   =>  "audio/wav",
        "opus"  =>  "audio/opus",
        "pcm"   =>  "audio/pcm",
        _ => "text/plain",
    };
    mime
}