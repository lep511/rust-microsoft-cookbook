use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LangsmithRequest {
    GetDataset(String),
    CreateDataset(RequestCreateDataset),
    CreateExample(RequestCreateExample),
    CreateModelPrice(RequestModel),
    GetRepo(RequestRepo),
    GetCommit(RequestCommit),
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

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestCreateExample {
    pub dataset_id: Option<String>,
    pub outputs: Option<Value>,
    pub inputs: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_urls: Option<Value>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestModel {
    pub name: String,
    pub prompt_cost: f64,
    pub completion_cost: f64,
    pub match_pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_path: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestRepo {
    pub owner: String,
    pub repo: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestCommit {
    pub owner: String,
    pub repo: String,
    pub commit: String,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Responses ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateDatasetResponse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    #[serde(default)]
    pub inputs_schema_definition: Option<Value>,
    #[serde(default)]
    pub outputs_schema_definition: Option<Value>,
    pub externally_managed: Option<bool>,
    #[serde(default)]
    pub transformations: Option<Vec<Value>>,
    pub data_type: Option<String>,
    pub id: Option<String>,
    pub tenant_id: Option<String>,
    pub example_count: Option<i64>,
    pub session_count: Option<i64>,
    pub modified_at: Option<String>,
    #[serde(default)]
    pub last_session_start_time: Option<String>,
    pub detail: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateExampleResponse {
    pub id: Option<String>,
    pub dataset_id: Option<String>,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub inputs: Option<Value>,
    pub outputs: Option<Value>,
    pub source_run_id: Option<String>,
    pub metadata: Option<Value>,
    pub name: Option<String>,
    pub attachment_urls: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelResponse {
    pub id: String,
    pub name: String,
    pub start_time: Option<String>,
    pub tenant_id: Option<String>,
    pub match_path: Vec<String>,
    pub match_pattern: String,
    pub prompt_cost: f64,
    pub completion_cost: f64,
    pub provider: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepoResponse {
    pub repo: Repository,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub repo_handle: Option<String>,
    pub description: Option<String>,
    pub readme: Option<String>,
    pub id: Option<String>,
    pub tenant_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub is_public: Option<bool>,
    pub is_archived: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub original_repo_id: Option<String>,
    pub upstream_repo_id: Option<String>,
    pub owner: Option<String>,
    pub full_name: Option<String>,
    pub num_likes: Option<i64>,
    pub num_downloads: Option<i64>,
    pub num_views: Option<i64>,
    pub last_commit_hash: Option<String>,
    pub num_commits: Option<i64>,
    pub original_repo_full_name: Option<String>,
    pub upstream_repo_full_name: Option<String>,
    pub latest_commit_manifest: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitResponse {
    pub commit_hash: String,
    pub manifest: Manifest,
    pub examples: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub id: Vec<String>,
    pub lc: i32,
    #[serde(rename = "type")]
    pub type_field: String,
    pub kwargs: ManifestKwargs,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManifestKwargs {
    pub messages: Vec<Message>,
    pub input_variables: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Vec<String>,
    pub lc: i32,
    #[serde(rename = "type")]
    pub type_field: String,
    pub kwargs: MessageKwargs,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageKwargs {
    pub prompt: Prompt,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub id: Vec<String>,
    pub lc: i32,
    #[serde(rename = "type")]
    pub type_field: String,
    pub kwargs: PromptKwargs,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromptKwargs {
    pub template: String,
    pub input_variables: Vec<String>,
    pub template_format: String,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Errors ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub detail: String,
}