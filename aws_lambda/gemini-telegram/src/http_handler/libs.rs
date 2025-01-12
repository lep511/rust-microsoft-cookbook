use serde::{Deserialize, Serialize};
use super::gen_config::GenerationConfig;
use super::errors::ErrorDetail;

#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct ChatRequest {
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<serde_json::Value>,
    #[serde(rename = "systemInstruction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,
    #[serde(rename = "generationConfig")]
    pub generation_config: Option<GenerationConfig>,
    #[serde(rename = "cachedContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_content: Option<String>,
    #[serde(rename = "safetySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functionCall", default)]
    pub function_call: Option<FunctionCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functionResponse", default)]
    pub function_response: Option<FunctionResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_data: Option<InlineData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<FileData>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InlineData {
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileData {
    pub mime_type: String,
    pub file_uri: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionResponse {
    pub name: String,
    pub response: FunctionContent,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionContent {
    pub name: String,
    pub content: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheRequest {
    pub model: String,
    pub contents: Vec<Content>,
    #[serde(rename = "systemInstruction")]
    pub system_instruction: Content,
    pub ttl: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedResponse {
    pub embedding: Option<Embedding>,
    pub error: Option<ErrorDetails>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Embedding {
    pub values: Vec<f32>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatResponse {
    pub candidates: Option<Vec<Candidate>>,
    pub model_version: Option<String>,
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: Option<UsageMetadata>,
    pub chat_history: Option<Vec<Content>>,
    pub error: Option<ErrorDetails>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candidate {
    pub content: Option<Content>,
    #[serde(rename = "finishReason")]
    pub finish_reason: Option<FinishReason>,
    #[serde(rename = "safetyRatings")]
    safety_ratings: Option<Vec<SafetyRating>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafetyRating {
    category: HarmCategory,
    probability: HarmProbability,
    blocked: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HarmCategory {
    #[serde(rename = "HARM_CATEGORY_UNSPECIFIED")]
    HarmCategoryUnspecified, // Category is unspecified.
    #[serde(rename = "HARM_CATEGORY_HARASSMENT")]
    HarmCategoryHarassment, // Harassment content.
    #[serde(rename = "HARM_CATEGORY_HATE_SPEECH")]
    HarmCategoryHateSpeech, // Hate speech and content.
    #[serde(rename = "HARM_CATEGORY_SEXUALLY_EXPLICIT")]
    HarmCategorySexuallyExplicit, // Sexually explicit content.
    #[serde(rename = "HARM_CATEGORY_DANGEROUS_CONTENT")]
    HarmCategoryDangerousContent, // Dangerous content.
    #[serde(rename = "HARM_CATEGORY_CIVIC_INTEGRITY")]
    HarmCategoryCivicIntegrity, // Content that may be used to harm civic integrity.
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HarmProbability {
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
    HarmProbabilityUnspecified, // Probability is unspecified.
    #[serde(rename = "NEGLIGIBLE")]
    Negligible, // Content has a negligible chance of being unsafe.
    #[serde(rename = "LOW")]
    Low, // Content has a low chance of being unsafe.
    #[serde(rename = "MEDIUM")]
    Medium, // Content has a medium chance of being unsafe.
    #[serde(rename = "HIGH")]
    High, // Content has a high chance of being unsafe.
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafetySetting {
    pub category: HarmCategory,
    pub threshold: HarmBlock,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HarmBlock {
    #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    HarmBlockThresholdUnspecified, // Probability is unspecified.
    #[serde(rename = "BLOCK_LOW_AND_ABOVE")]
    BlockLowAndAbove, // Content has a negligible chance of being unsafe.
    #[serde(rename = "BLOCK_MEDIUM_AND_ABOVE")]
    BlockMediumAndAbove, // Content has a low chance of being unsafe.
    #[serde(rename = "BLOCK_ONLY_HIGH")]
    BlockOnlyHigh, // Content has a medium chance of being unsafe.
    #[serde(rename = "BLOCK_NONE")]
    BlockNone, // Content has a high chance of being unsafe.
    #[serde(rename = "OFF")]
    Off, // Turn off the safety filter.
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FinishReason {
    #[serde(rename = "FINISH_REASON_UNSPECIFIED")] // Default value. This value is unused
    Unspecified,
    #[serde(rename = "STOP")] // Natural stop point of the model or provided stop sequence
    Stop,
    #[serde(rename = "MAX_TOKENS")] // The maximum number of tokens as specified in the request was reached
    MaxTokens,
    #[serde(rename = "SAFETY")] // The response candidate content was flagged for safety reasons
    Safety,
    #[serde(rename = "RECITATION")] // 	he response candidate content was flagged for recitation reasons
    Recitation,
    #[serde(rename = "LANGUAGE")] // The response candidate content was flagged for using an unsupported language
    Language,
    #[serde(rename = "OTHER")] // Unknown reason.
    Other,
    #[serde(rename = "BLOCKLIST")] // Token generation stopped because the content contains forbidden terms
    Blocklist,
    #[serde(rename = "PROHIBITED_CONTENT")] // Token generation stopped for potentially containing prohibited content
    ProhibitedContent,
    #[serde(rename = "SPII")] // Token generation stopped because the content potentially contains Sensitive Personally Identifiable Information (SPII)
    Spii,
    #[serde(rename = "MALFORMED_FUNCTION_CALL")] // The function call generated by the model is invalid
    MalformedFunctionCall,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsageMetadata {
    #[serde(rename = "candidatesTokenCount")]
    pub candidates_token_count: Option<i32>,
    #[serde(rename = "promptTokenCount")]
    pub rompt_token_count: Option<i32>,
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: Option<i32>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorDetails {
    pub code: Option<i32>,
    pub message: Option<String>,
    pub status: Option<String>,
    pub details: Option<Vec<ErrorDetail>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedRequest {
    pub model: String,
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dimensionality: Option<i32>,
    pub task_type: TaskType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

// Choose an embeddings task type:
// https://cloud.google.com/vertex-ai/generative-ai/docs/embeddings/task-types
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskType {
    #[serde(rename = "TASK_TYPE_UNSPECIFIED")] // If you do not set the value, it will default to retrieval_query.
    Unspecified,
    #[serde(rename = "RETRIEVAL_QUERY")] // The given text is a query in a search/retrieval setting.
    RetrievalQuery,
    #[serde(rename = "RETRIEVAL_DOCUMENT")] //  The given text is a document from the corpus being searched.
    RetrievalDocument,
    #[serde(rename = "SEMANTIC_SIMILARITY")] // The given text will be used for Semantic Textual Similarity (STS).
    SemanticSimilarity,
    #[serde(rename = "CLASSIFICATION")] // The given text will be classified.
    Classification,
    #[serde(rename = "CLUSTERING")] // The embeddings will be used for clustering.
    Clustering,
    #[serde(rename = "QUESTION_ANSWERING")] // The given text will be used for question answering.
    QuestionAnswering,
    #[serde(rename = "FACT_VERIFICATION")] // The given text will be used for fact verification
    FactVerification,
    #[serde(rename = "CODE_RETRIEVAL_QUERY")] // The given text is a query in a code retrieval setting.
    CodeRetrievalQuery,
}