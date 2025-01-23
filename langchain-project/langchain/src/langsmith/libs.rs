use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LangsmithRequest {
    CreateDataset(RequestCreateDataset),
    Unknown,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LangsmithResponse {
    CreateDataset(CreateDatasetResponse),
    Unknown,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Requests ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestCreateDataset {
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs_schema_definition: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs_schema_definition: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub externally_managed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations: Option<Vec<Transformation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transformation {
    pub path: Option<Vec<PathValue>>,
    pub transformation_type: Option<TransformationType>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathValue {
    pub value: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformationType {
    pub value: Option<String>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Responses ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateDatasetResponse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub inputs_schema_definition: Option<Value>,
    pub outputs_schema_definition: Option<Value>,
    pub externally_managed: Option<bool>,
    pub transformations: Option<Vec<Value>>,
    pub data_type: Option<String>,
    pub id: Option<String>,
    pub tenant_id: Option<String>,
    pub example_count: Option<i64>,
    pub session_count: Option<i64>,
    pub modified_at: Option<String>,
    pub last_session_start_time: Option<String>,
    pub detail: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub detail: String,
}