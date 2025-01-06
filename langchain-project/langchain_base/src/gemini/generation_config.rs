use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  GenerationConfig {
    pub temperature: Option<f32>,
    #[serde(rename = "topK")]
    pub top_k: Option<u32>,
    #[serde(rename = "topP")]
    pub top_p: Option<f32>,
    #[serde(rename = "maxOutputTokens")]
    pub max_output_tokens: Option<u32>,
    #[serde(rename = "responseMimeType")]
    pub response_mime_type: Option<String>,
    #[serde(rename = "responseSchema")]
    pub response_schema: Option<serde_json::Value>,
    #[serde(rename = "stopSequences")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(rename = "candidateCount")]
    pub candidate_count: Option<u32>,
    #[serde(rename = "presencePenalty")]
    pub presence_penalty: Option<f32>,
    #[serde(rename = "frequencyPenalty")]
    pub frequency_penalty: Option<f32>,
    #[serde(rename = "responseLogprobs")]
    pub response_logprobs: Option<bool>,
    #[serde(rename = "logProbs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_probs: Option<u32>,
}