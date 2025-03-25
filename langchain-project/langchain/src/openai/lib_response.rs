use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseRequest {
    pub input: Input,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>, // ToDo - Change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>, // ToDo - Change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning:  Option<Reasoning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store:  Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<String>,  // ToDo - Change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Input {
    String(String),
    Image(Vec<Message>),
    File(Vec<Message>),
    Conversation(Vec<Message>),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolChoice {
    String(String),
    Object(Value),
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct Format {
    pub format: Value,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct Reasoning {
    pub effort: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_turn: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Platform,
    Developer,
    User,
    Assistant,
    Tool,
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
    pub image_url: Option<ImageUrl>,
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrl {
    pub url: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ResponseObject {
    pub created_at: Option<u64>,
    pub error: Option<ErrorDetails>,
    pub id: Option<String>,
    pub incomplete_details: Option<String>, // ToDo - Change
    pub instructions: Option<String>,
    pub max_output_tokens: Option<u32>,
    pub metadata: Option<Metadata>,
    pub model: Option<String>,
    pub object: Option<String>,
    pub output: Option<String>,  // ToDo - Change
    pub output_text: Option<String>,
    pub parallel_tool_calls: Option<bool>,
    pub previous_response_id: Option<String>,
    pub reasoning: Option<Reasoning>,
    pub status: Option<String>,
    pub temperature: Option<f32>,
    pub text: Option<Format>,
    pub tool_choice: Option<ToolChoice>,
    pub tools: Option<Value>,
    pub top_p: Option<f32>,
    pub truncation: Option<String>,
    pub usage: Option<Usage>,
    pub user: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub code: String, // ToDo - Change
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
}