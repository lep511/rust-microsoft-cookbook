use reqwest::Client;
use crate::langsmith::libs::{
    LangsmithRequest, CreateDatasetResponse, CreateExampleResponse,
    LangsmithResponse, ErrorResponse,
};
use crate::langsmith::utils::print_pre;
use crate::langsmith::{LANGSMITH_BASE_URL, DEBUG_PRE, DEBUG_POST};
use crate::llmerror::LangsmithError;
use serde_json::Value;

pub async fn request_langsmith(
    request: &LangsmithRequest,
    api_key: &str,
) -> Result<LangsmithResponse, Box<dyn std::error::Error>> {

    let response_enum: LangsmithResponse;

    match request {
        LangsmithRequest::GetDataset(dataset_name) => {
            let url = format!("{}/datasets/?name={}", LANGSMITH_BASE_URL, dataset_name);
            let response_string = get_request(
                &url,
                api_key
            ).await?;

            if response_string == "null".to_string() {
                return Ok(LangsmithResponse::Empty);
            } else {
                let response: CreateDatasetResponse = match serde_json::from_str(&response_string) {
                    Ok(response_form) => response_form,
                    Err(e) => {
                        return Err(Box::new(LangsmithError::GenericError {
                            message: format!("Failed to process response: {}", e),
                            detail: "ERROR-req-9881".to_string(),
                        }));
                    }
                };

                return Ok(LangsmithResponse::CreateDataset(response));
            }
        },
        LangsmithRequest::CreateDataset(request_create_dataset) => {
            let url = format!("{}/datasets", LANGSMITH_BASE_URL);
            let request_json = serde_json::to_value(&request_create_dataset)?;
            let response_string = post_request(
                request_json, 
                &url, 
                api_key
            ).await?;

            let response: CreateDatasetResponse = match serde_json::from_str(&response_string) {
                Ok(response_form) => response_form,
                Err(e) => {
                    println!("[ERROR] {:?}", e);
                    return Err(Box::new(LangsmithError::GenericError {
                        message: "Failed to process response".to_string(),
                        detail: "ERROR-req-9889".to_string(),
                    }));
                }
            };

            response_enum = LangsmithResponse::CreateDataset(response);
        },
        LangsmithRequest::CreateExample(request_create_example)  => {
            let url = format!("{}/examples", LANGSMITH_BASE_URL);
            let request_json = serde_json::to_value(&request_create_example)?;
            let response_string = post_request(
                request_json, 
                &url, 
                api_key
            ).await?;

            let response: CreateExampleResponse = match serde_json::from_str(&response_string) {
                Ok(response_form) => response_form,
                Err(e) => {
                    println!("[ERROR] {:?}", e);
                    return Err(Box::new(LangsmithError::GenericError {
                        message: "Failed to process response".to_string(),
                        detail: "ERROR-req-9898".to_string(),
                    }));
                }
            };

            response_enum = LangsmithResponse::CreateExample(response);
        },
        LangsmithRequest::Unknown => {
            return Err(Box::new(LangsmithError::GenericError {
                message: "Unknown request type".to_string(),
                detail: "ERROR-req-9890".to_string(),
            }));
        }
    } 
    
    Ok(response_enum)
}

pub async fn get_request(url: &str, api_key: &str) -> Result<String, LangsmithError> {
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
        return Err(LangsmithError::GenericError {
            message: error_response.detail,
            detail: "ERROR-req-9877".to_string(),
        });
    }

    let response_data = response.json::<serde_json::Value>().await?;

    print_pre(&response_data, DEBUG_POST);

    let response_string = response_data[0].to_string();

    Ok(response_string)
}

pub async fn post_request(request: Value, url: &str, api_key: &str) -> Result<String, LangsmithError> {
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
        return Err(LangsmithError::GenericError {
            message: error_response.detail,
            detail: "ERROR-req-9880".to_string(),
        });
    }
    
    let response_data = response.json::<serde_json::Value>().await?;

    print_pre(&response_data, DEBUG_POST);

    let response_string = response_data.to_string();
    
    Ok(response_string)
}