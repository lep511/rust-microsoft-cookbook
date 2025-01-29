use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a request to the chat API
///
/// # Fields
/// * `model` - Optional. The name of the model to use
/// * `messages` - Optional. Array of messages comprising the conversation history
/// * `input` - Optional. The input value to process
/// * `temperature` - Optional. Controls randomness in response generation 
/// * `max_tokens` - Optional. Maximum number of tokens to generate
/// * `tools` - Optional. Array of available tools/functions that can be called
/// * `tool_choice` - Optional. Specification for tool selection behavior
/// * `frequency_penalty` - Optional. Penalizes frequent tokens
/// * `presence_penalty` - Optional. Penalizes repeated topics
/// * `top_p` - Optional. Nucleus sampling threshold
/// * `min_p` - Optional. The minimum probability for a token to be considered.
/// * `top_k` - Optional. Number of highest probability tokens to consider
/// * `stop` - Optional. Array of sequences where generation should stop
/// * `n_completion` - Optional. Number of chat completion choices to generate
/// * `response_format` - Optional. The format of the response.
/// * `stream` - Optional. Whether to stream responses or return complete response
#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct ChatRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "n")]
    pub n_completion: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

/// Represents a message in a conversation
///
/// # Fields
/// * `role` - Optional. The role of the message sender (e.g., "system", "user", "assistant")
/// * `content` - Optional. The actual text content of the message
/// * `tool_calls` - Optional. Array of tool calls made within this message
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Option<String>,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<Value>>,
}

/// Represents a response from the chat API
///
/// # Fields
/// * `choices` - Optional. Array of generated response choices
/// * `id` - Optional. Unique identifier for the response
/// * `created` - Optional. Unix timestamp of when the response was created
/// * `model` - Optional. Name of the model used to generate the response
/// * `object` - Optional. Type of object returned
/// * `system_fingerprint` - Optional. System identifier or version information
/// * `usage` - Optional. Token usage statistics for the request/response
/// * `chat_history` - Optional. Array of messages showing conversation history
/// * `error` - Optional. Details of any error that occurred during processing
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Option<Vec<Choice>>,
    pub id: Option<String>,
    pub created: Option<u64>,
    pub model: Option<String>,
    pub object: Option<String>,
    pub system_fingerprint: Option<String>,
    pub usage: Option<Usage>,
    pub chat_history: Option<Vec<Message>>,
    pub error: Option<ErrorDetails>,
}

/// Represents a single response choice from the chat API
///
/// # Fields
/// * `finish_reason` - Optional. Reason why the response generation was completed
/// * `delta` - Optional. Incremental message update when streaming responses
/// * `index` - Optional. Index of this choice in the array of choices
/// * `logprobs` - Optional. Log probabilities for token generation
/// * `message` - Optional. The complete response message for this choice
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub finish_reason: Option<String>,
    pub delta: Option<Message>,
    pub index: Option<u32>,
    pub logprobs: Option<serde_json::Value>,
    pub message: Option<Message>,
}

/// Represents usage statistics for an API request/response
///
/// # Fields
/// * `completion_time` - Optional. Time taken to generate the completion in seconds
/// * `completion_tokens` - Optional. Number of tokens in the generated completion
/// * `prompt_time` - Optional. Time taken to process the prompt in seconds
/// * `prompt_tokens` - Optional. Number of tokens in the prompt
/// * `queue_time` - Optional. Time spent in queue before processing in seconds
/// * `total_time` - Optional. Total processing time in seconds
/// * `total_tokens` - Optional. Total number of tokens used (prompt + completion)
/// * `prompt_tokens_details` - Optional. Detailed breakdown of prompt token usage
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Usage {
    pub completion_time: Option<f64>,
    pub completion_tokens: Option<u32>,
    pub prompt_time: Option<f64>,
    pub prompt_tokens: Option<u32>,
    pub queue_time: Option<f64>,
    pub total_time: Option<f64>,
    pub total_tokens: Option<u32>,
    pub prompt_tokens_details: Option<PromptTokensDetails>,
}

/// Provides a detailed breakdown of token usage by content type
///
/// # Fields
/// * `text_tokens` - Optional. Number of tokens used for text content
/// * `audio_tokens` - Optional. Number of tokens used for audio content
/// * `image_tokens` - Optional. Number of tokens used for image content
/// * `cached_tokens` - Optional. Number of tokens retrieved from cache
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PromptTokensDetails {
    pub text_tokens: Option<u32>,
    pub audio_tokens: Option<u32>,
    pub image_tokens: Option<u32>,
    pub cached_tokens: Option<u32>,
}

/// Contains details about an error that occurred during API processing
///
/// # Fields
/// * `code` - Optional. Error code identifier
/// * `message` - The error message describing what went wrong
/// * `param` - Optional. Parameter that caused the error, if applicable
/// * `error_type` - The type/category of error (renamed from "type" due to Rust keyword)
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
    #[serde(rename = "type")]
    pub error_type: String,
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Errors ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Represents an error response structure
/// 
/// This struct is used to provide a standardized error response format,
/// containing a detailed error message.
///
/// # Fields
/// * `detail` - Detailed error message or description of what went wrong
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub detail: String,
}