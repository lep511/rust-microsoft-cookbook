use std::env;
use crate::llmerror::GeminiError;
use crate::gemini::libs::Candidate;
use lambda_runtime::tracing;

/// Gets the API key from the environment variables
///
/// # Returns
/// * A string slice containing the API key
///
/// # Panics
/// * If the GEMINI_API_KEY environment variable is not set
///
pub trait GetApiKey {
    fn get_api_key() -> Result<String, GeminiError> {
        match env::var("GEMINI_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                tracing::error!("Error GEMINI_API_KEY not found in environment variables");
                Err(GeminiError::ApiKeyNotFound)
            }
            Err(e) => {
                tracing::error!("Error {:?}", e);
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
            Err(e) => tracing::error!("Error {:?}", e)
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

pub fn get_grounding_response(candidate: &Candidate) -> String {
    if let Some(grounding_metadata) = &candidate.grounding_metadata {
        let mut markdown_text = String::new();

        if let Some(supports) = &grounding_metadata.grounding_supports {
            for support in supports {
                if let (Some(segment), Some(chunk_indices)) = (&support.segment, &support.grounding_chunk_indices) {
                    if let Some(_end_index) = segment.end_index {
                        if let Some(text) = &segment.text {
                            markdown_text.push_str(text);

                            // Add footnotes
                            for &chunk_index in chunk_indices {
                                if let Some(chunks) = &grounding_metadata.grounding_chunks {
                                    if let Some(chunk) = chunks.get(chunk_index as usize) {
                                        if let Some(web_info) = &chunk.web {
                                            markdown_text.push_str(&format!("[[{}]]({}))\n", 
                                                chunk_index + 1, 
                                                web_info.uri
                                            ));
                                        }
                                    }
                                }
                            }
                            markdown_text.push('\n');
                        }
                    }
                }
            }
        }

        markdown_text.push_str("\n----\n## Grounding Sources\n");

        // Add web search queries if present
        if let Some(web_queries) = &grounding_metadata.web_search_queries {
            markdown_text.push_str(&format!("\n**Web Search Queries:** {:?}\n", web_queries));
            
            if let Some(entry_point) = &grounding_metadata.search_entry_point {
                if let Some(content) = &entry_point.rendered_content {
                    markdown_text.push_str(&format!("\n**Search Entry Point:**\n {}\n", content));
                }
            }
        }

        markdown_text.push_str("### Grounding Chunks\n");

        // Add grounding chunks
        if let Some(chunks) = &grounding_metadata.grounding_chunks {
            for (index, chunk) in chunks.iter().enumerate() {
                if let Some(web_info) = &chunk.web {
                    markdown_text.push_str(&format!("{}. [{}]({})\n", 
                        index + 1,
                        web_info.title,
                        web_info.uri
                    ));
                }
            }
        }

        markdown_text
    } else {
        String::from("No grounding metadata available")
    }
}