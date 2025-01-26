use crate::langsmith::libs::{
    LangsmithRequest, RequestCreateDataset, RequestCreateExample,
    RequestModel, RequestRepo, RequestCommit,
};
use crate::langsmith::utils::GetApiKey;
use crate::langsmith::requests::request_langsmith;
use crate::llmerror::LangsmithError;
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LangsmithClient {
    pub request: LangsmithRequest,
    pub api_key: String,
}

#[allow(dead_code)]
impl LangsmithClient {
    pub fn new() -> Result<Self, LangsmithError> {
        let api_key = Self::get_api_key()?;
       
        Ok(Self {
            request: LangsmithRequest::Unknown,
            api_key: api_key,
        })
    }

    pub async fn invoke(self) -> Result<Value, LangsmithError> {
        
        let response: Value = match request_langsmith(
            &self.request,
            &self.api_key,
        ).await {
            Ok(response) => response,
            Err(e) => {
                println!("[ERROR] {:?}", e);
                return Err(LangsmithError::ResponseContentError);
            }
        };

        Ok(response)
    }

    pub fn get_dataset(mut self, dataset_name: &str) -> Self {
        self.request = LangsmithRequest::GetDataset(dataset_name.to_string());
        self
    }

    pub fn create_dataset(mut self, name: &str) -> Self {

        let request_create_dataset = RequestCreateDataset {
            name: Some(name.to_string()),
            description: None,
            created_at: None,
            inputs_schema_definition: None,
            outputs_schema_definition: None,
            externally_managed: None,
            transformations: None,
            id: None,
            extra: None,
            data_type: None,
        };

        self.request = LangsmithRequest::CreateDataset(request_create_dataset);

        self
    }

    pub fn create_example(
        mut self,
        dataset_id: &str,
        input: serde_json::Value,
        output: serde_json::Value,
    ) -> Self {

        let request_create_example = RequestCreateExample {
            outputs: Some(input),
            dataset_id: Some(dataset_id.to_string()),
            source_run_id: None,
            metadata: None,
            inputs: Some(output),
            created_at: None,
            id: None,
            name: None,
            modified_at: None,
            attachment_urls: None,
        };

        self.request = LangsmithRequest::CreateExample(request_create_example);

        self
    }

    pub fn create_model_price(
        mut self,
        model_name: &str,
        prompt_cost: f64,
        completion_cost: f64,
        match_pattern: &str,
        start_time: Option<&str>,
        match_path: Option<Vec<String>>,
        provider: Option<&str>,
    ) -> Self {

        let request_model_price = RequestModel {
            name: model_name.to_string(),
            prompt_cost: prompt_cost,
            completion_cost: completion_cost,
            match_pattern: match_pattern.to_string(),
            start_time: start_time.map(|s| s.to_string()),
            match_path: match_path,
            provider: provider.map(|s| s.to_string()),
        };

        self.request = LangsmithRequest::CreateModelPrice(request_model_price);

        self
    }

    pub fn get_repo(
        mut self,
        owner: &str,
        repo: &str,
    ) -> Self {

        let request_repo = RequestRepo {
            owner: owner.to_string(),
            repo: repo.to_string(),
        };

        self.request = LangsmithRequest::GetRepo(request_repo);

        self
    }

    pub fn get_commit(
        mut self,
        owner: &str,
        repo: &str,
        commit: &str,
    ) -> Self {

        let request_commit = RequestCommit {
            owner: owner.to_string(),
            repo: repo.to_string(),
            commit: commit.to_string(),
        };

        self.request = LangsmithRequest::GetCommit(request_commit);

        self
    }
        
    // pub fn create_examples(
    //     mut self,
    //     dataset_id: &str,
    //     examples: Vec<(&str, &str)>, 
    //     label_input: &str, 
    //     label_output: &str
    // ) -> Self {

    // let input: Vec<serde_json::Value> = examples
    //     .iter()
    //     .map(|(text, _)| {
    //         serde_json::json!({
    //             label_input: text
    //         })
    //     })
    //     .collect();
    
    // let output: Vec<serde_json::Value> = examples
    //     .iter()
    //     .map(|(_, label)| {
    //         serde_json::json!({
    //             label_output: label
    //         })
    //     })
    //     .collect();
        
    //     let request_create_example = RequestCreateExample {
    //         outputs: Some(output),
    //         dataset_id: Some(dataset_id),
    //         source_run_id: None,
    //         metadata: None,
    //         inputs: Some(input),
    //         created_at: None,
    //         id: None,
    //         name: None,
    //         modified_at: None,
    //         attachment_urls: None,
    //     };

    //     self.request = LangsmithRequest::CreateExample(request_create_example);

    //     self
    // }
    
    pub fn with_description(mut self, description: &str) -> Self {
        match &mut self.request {
            LangsmithRequest::CreateDataset(request) => {
                request.description = Some(description.to_string());
            }
            _ => {}
        }
        self
    }

}

impl GetApiKey for LangsmithClient {}