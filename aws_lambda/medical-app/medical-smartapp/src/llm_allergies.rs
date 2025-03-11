use crate::gemini::chat::ChatGemini;
use crate::oidc_request::get_mdata;
use crate::libs::MainPageParams;
use serde::{Deserialize, Serialize};
use lambda_runtime::tracing::error;
use serde_json::{Value, json};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Allergies {
    pub entrys: Vec<AllergyItem>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AllergyItem {
    pub code_display: String,
    pub severity: String,
}

pub async fn extract_allergies(
    params: &MainPageParams,
) -> Vec<String> {
    let allergies = match extract_allergies_handle(params).await {
        Ok(allergies) => allergies,
        Err(e) => {
            error!("Error extracting allergies: {:?}", e);
            vec![]
        }
    };

    allergies
}

pub async fn extract_allergies_handle(
    params: &MainPageParams,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {

    let query = format!("AllergyIntolerance?patient={}", params.patient_id);

    let allergies = match get_mdata(
        &params.iss,
        &query,
        &params.access_token,
    ).await {
        Ok(data) => data,
        Err(e) => {
            error!("Error getting AllergyIntolerance data: {:?}", e);
            return Err(Box::new(e));
        }
    };

    let llm = ChatGemini::new("gemini-2.0-flash");

    let get_allergy_intolerance = json!({
    "name":"get_allergy_intolerance",
    "description":"Function to extract entries related to allergies.",
    "parameters":{
        "type":"OBJECT",
        "properties":{
            "entrys":{
                "type":"ARRAY",
                "description":"List of allergy entries",
                "items":{
                    "type":"OBJECT",
                    "properties":{
                        "severity":{
                            "type":"STRING",
                            "description":"Severity of allergy"
                        },
                        "code_display":{
                            "type":"STRING",
                            "description":"Display code of the allergy"
                        }
                    },
                    "required":[
                        "severity",
                        "code_display"
                    ]
                }
            }
        }
    }
    });

    let function_dec = vec![json!({
        "functionDeclarations":[
            get_allergy_intolerance
        ]
    })];

    let tool_config = json!({
        "function_calling_config":{
            "mode":"ANY",
            "allowed_function_names":[
                "get_allergy_intolerance"
            ]
        }
    });

    let prompt = format!(
        "You will be analyzing a FHIR (Fast Healthcare Interoperability Resources) Bundle \
        containing AllergyIntolerance resources. The Bundle is provided in JSON format. \
        Your task is to extract all information from the entrys resources: \
        \n \
        Here is the FHIR Bundle: \
        \n \
        {}", allergies);

    let response = llm
        .with_tools(function_dec)
        .with_tool_config(tool_config)
        .invoke(&prompt)
        .await?;

    let mut allergies_array = Vec::new();
    let mut function_args = Value::Null;

    if let Some(candidates) = &response.candidates {
        for candidate in candidates {
            if let Some(content) = &candidate.content {
                for part in &content.parts {
                    if let Some(function_call) = &part.function_call {
                        function_args = function_call.args.clone();
                    }
                }
            }
        }
    };
    
    let get_allergies: Allergies = serde_json::from_value(function_args)?;
    for alle in get_allergies.entrys {
        allergies_array.push(alle.code_display);
    }
    Ok(allergies_array)
}