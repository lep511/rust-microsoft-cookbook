use crate::langsmith::libs::{
    LangsmithRequest, RequestCreateDataset, LangsmithResponse
};
use crate::langsmith::utils::GetApiKey;
use crate::langsmith::requests::request_langsmith;
use crate::llmerror::LangsmithError;

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

    pub async fn invoke(self) -> Result<LangsmithResponse, LangsmithError> {
        
        let response: LangsmithResponse = match request_langsmith(
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