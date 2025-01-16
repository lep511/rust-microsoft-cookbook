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

pub fn check_mimetype(mime: &str) -> bool {
    let accepted_mime_type = [
        "image/jpeg",
        "image/jpg",
        "image/png",
        "image/webp",
        "image/gif",
        "video/mp4",
        "video/x-flv",
        "video/quicktime",
        "video/mpeg",
        "video/3gpp",
        "video/webm",
        "video/x-ms-wmv",
        "application/pdf",
        "application/msword",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "application/rtf",
        "text/plain",
        "text/csv",
        "text/tab-separated-values",
        "application/vnd.ms-excel",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "audio/mpeg",
        "audio/aac",
        "audio/wav",
        "audio/opus",
        "audio/pcm",
    ];

    // Check if mime is in accepted_mime_type array
    !accepted_mime_type.iter().any(|&x| x == mime)
}

pub fn get_base64_bytes_length(base64_str: &str) -> usize {
    let padding_count = base64_str.chars().filter(|&c| c == '=').count();
    (base64_str.len() * 3 / 4) - padding_count
}