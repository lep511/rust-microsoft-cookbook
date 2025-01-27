use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct ChatRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,
    pub stream: bool,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // Allows for multiple types of input
pub enum InputEmbed {
    Array(Vec<String>),
    String(String),
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<InputEmbed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<EmbedContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dimension: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,  
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AnthropicEmbedEndpoint {
    Embed,
    MultimodalEmbed,
    Rerank,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedContent {
    pub content: Vec<Content>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedResponse {
    pub data: Option<Vec<EmbeddingData>>,
    pub model: Option<String>,
    pub object: Option<String>,
    pub usage: Option<Usage>,
    pub detail: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingData {
    pub embedding: Option<Vec<f64>>,
    pub index: Option<usize>,
    pub object: Option<String>,
    pub relevance_score: Option<f64>,
    pub document: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: Vec<Content>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_base64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_use_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Source {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub content: Option<Vec<Content>>,
    pub id: Option<String>,
    pub model: Option<String>,
    pub role: Option<String>,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    #[serde(rename = "type")]
    pub response_type: String,
    pub usage: Option<Usage>,
    pub chat_history: Option<Vec<Message>>,
    pub error: Option<ErrorDetails>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub cache_creation_input_tokens: Option<u32>,
    pub cache_read_input_tokens: Option<u32>,
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
    pub image_pixels: Option<u32>,
    pub text_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorDetails {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
}