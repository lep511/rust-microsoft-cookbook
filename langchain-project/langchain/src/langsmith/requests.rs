use reqwest::Client;
use log::error;
use crate::langsmith::libs::{LangsmithRequest, ErrorResponse};
use crate::langsmith::utils::print_pre;
use crate::langsmith::{LANGSMITH_BASE_URL, DEBUG_PRE, DEBUG_POST};
use crate::llmerror::LangsmithError;
use serde_json::Value;

pub async fn request_langsmith(
    request: &LangsmithRequest,
    api_key: &str,
) -> Result<Value, Box<dyn std::error::Error>> {

    let response_value;

    match request {
        LangsmithRequest::GetDataset(dataset_name) => {
            let url = format!("{}/datasets/?name={}", LANGSMITH_BASE_URL, dataset_name);
            response_value = get_request(
                &url,
                api_key
            ).await?;            
        },
        LangsmithRequest::CreateDataset(request_create_dataset) => {
            let url = format!("{}/datasets", LANGSMITH_BASE_URL);
            let request_json = serde_json::to_value(&request_create_dataset)?;
            response_value = post_request(
                request_json, 
                &url, 
                api_key
            ).await?;
        },
        LangsmithRequest::CreateExample(request_create_example)  => {
            let url = format!("{}/examples", LANGSMITH_BASE_URL);
            let request_json = serde_json::to_value(&request_create_example)?;
            response_value = post_request(
                request_json, 
                &url, 
                api_key
            ).await?;
        },
        LangsmithRequest::CreateModelPrice(request_model) => {
            let url = format!("{}/model-price-map", LANGSMITH_BASE_URL);
            let request_json = serde_json::to_value(&request_model)?;
            response_value = post_request(
                request_json, 
                &url, 
                api_key
            ).await?;
        },
        LangsmithRequest::GetRepo(request_repo) => {
            let url = format!(
                "{}/repos/{}/{}", 
                LANGSMITH_BASE_URL, 
                request_repo.owner,
                request_repo.repo,
            );
            response_value = get_request(
                &url,
                api_key
            ).await?;
        },
        LangsmithRequest::GetCommit(request_commit) => {
            let url = format!(
                "{}/commits/{}/{}/{}",
                LANGSMITH_BASE_URL,
                request_commit.owner,
                request_commit.repo,
                request_commit.commit,
            );
            response_value = get_request(
                &url,
                api_key
            ).await?;
        },
        LangsmithRequest::Unknown => {
            error!("Unknown request type");
            return Err(Box::new(LangsmithError::GenericError {
                message: "Unknown request type".to_string(),
                detail: "ERROR-req-9890".to_string(),
            }));
        }
    } 
    
    Ok(response_value)
}

pub async fn get_request(url: &str, api_key: &str) -> Result<Value, LangsmithError> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let response = client
        .request(reqwest::Method::GET, url)
        .header("X-API-Key", api_key)
        .header("Accept", "application/json")
        .send()
        .await?;

    if !response.status().is_success() {
        let error_response = response.json::<ErrorResponse>().await?;
        error!("Error response: {:?}", error_response);
        return Err(LangsmithError::GenericError {
            message: error_response.detail,
            detail: "ERROR-req-9877".to_string(),
        });
    }

    let response_data = response.json::<serde_json::Value>().await?;

    print_pre(&response_data, DEBUG_POST);

    Ok(response_data)
}

pub async fn post_request(request: Value, url: &str, api_key: &str) -> Result<Value, LangsmithError> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    print_pre(&request, DEBUG_PRE);

    let response = client
        .post(url)
        .header("X-API-Key", api_key)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;
          
    if !response.status().is_success() {
        let error_response = response.json::<ErrorResponse>().await?;
        error!("Error response: {:?}", error_response);
        return Err(LangsmithError::GenericError {
            message: error_response.detail,
            detail: "ERROR-req-9880".to_string(),
        });
    }
    
    let response_data = response.json::<serde_json::Value>().await?;

    print_pre(&response_data, DEBUG_POST);
    
    Ok(response_data)
}