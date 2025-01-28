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

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Option<String>,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<Value>>,
}

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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub finish_reason: Option<String>,
    pub delta: Option<Message>,
    pub index: Option<u32>,
    pub logprobs: Option<serde_json::Value>,
    pub message: Option<Message>,
}

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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PromptTokensDetails {
    pub text_tokens: Option<u32>,
    pub audio_tokens: Option<u32>,
    pub image_tokens: Option<u32>,
    pub cached_tokens: Option<u32>,
}

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

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub detail: String,
}