use lambda_runtime::{tracing, Error, LambdaEvent};
use aws_sdk_s3::Client;
use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use crate::gemini::chat::ChatGemini;
use crate::mongodb::connect::mongodb_update;
use crate::libs::{FileData, MedicalDummie};
use serde_json::from_str;
use crate::llmerror::S3Error;

pub async fn get_s3_object(
    s3_client: &Client, 
    bucket: &str, 
    object_key: &str
) -> Result<String, S3Error> {

    tracing::info!("bucket: {}", bucket);
    tracing::info!("object: {}", object_key);
    
    let object = s3_client.get_object()
        .bucket(bucket)
        .key(object_key)
        .send()
        .await?;

    let mut content = String::new();
    let mut stream = object.body;
    
    while let Some(bytes) = stream.try_next().await.map_err(|err| {
        S3Error::new(format!("Failed to read from S3 download stream: {err:?}"))
    })? {
        content.push_str(&String::from_utf8(bytes.to_vec()).map_err(|err| {
            S3Error::new(format!("Failed to convert bytes to UTF-8 string: {err:?}"))
        })?);
    }

    Ok(content)
}

pub async fn move_s3_object(
    s3_client: &Client,
    input_bucket: &str,
    object_key: &str,
    output_bucket: &str
) -> Result<(), S3Error> {

    let copy_source = format!("{}/{}", input_bucket, object_key);
    let output_key = format!("process/{}", object_key);

    s3_client.copy_object()
        .bucket(output_bucket)
        .copy_source(copy_source)
        .key(&output_key)
        .send()
        .await?;

    s3_client.delete_object()
        .bucket(input_bucket)
        .key(object_key)
        .send()
        .await?;

    Ok(())
}

pub(crate)async fn function_handler(event: LambdaEvent<EventBridgeEvent>) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload = event.payload;
    // tracing::info!("Payload: {:?}", payload);

    let bucket = std::env::var("INPUT_BUCKET")
        .expect("INPUT_BUCKET environment variable not set.");

    let output_bucket = std::env::var("OUTPUT_BUCKET")
        .expect("OUTPUT_BUCKET environment variable not set.");

    let object_key = payload
        .detail
        .get("object")
        .and_then(|obj| obj.get("key"))
        .and_then(|key| key.as_str())
        .ok_or_else(|| Error::from("Failed to get object key".to_string()))?;

    // ~~~~~~~~~~~~~~~~~~~~~~ Get File-content from Bucket ~~~~~~~~~~~~~~~~~~~~~~~~~~

    let shared_config = aws_config::load_from_env().await;
    let s3_client = Client::new(&shared_config);

    let object_content = match get_s3_object(&s3_client, &bucket, object_key).await {
        Ok(content) => content,
        Err(err) => {
            tracing::error!("Error getting S3 object: {:?}", err);
            return Err(Error::from(err.to_string()));
        }
    };

    let file_data: FileData = from_str(&object_content)?;

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Gemini Response ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let llm = ChatGemini::new("gemini-2.0-flash-thinking-exp-01-21");

    let system_prompt = "Pretend that you are a professional medical coder";

    let examples = "{'\''medical_terms'\'': [{'\''code_type'\'': '\''ICD-10'\'',\n'\''code_value'\'': '\''J44.9'\'',\n'\''code_explain'\'': '\''COPD with exacerbation'\''},\n{'\''code_type'\'': '\''ICD-10'\'',\n'\''code_value'\'': '\''J96.91'\'',\n'\''code_explain'\'': '\''Respiratory failure, unspecified'\''},\n{'\''code_type'\'': '\''ICD-10'\'',\n'\''code_value'\'': '\''J81.0'\'',\n'\''code_explain'\'': '\''Pulmonary edema'\''},\n{'\''code_type'\'': '\''ICD-10'\'',\n'\''code_value'\'': '\''D63.81'\'',\n'\''code_explain'\'': '\''Microcytic anemia'\''},\n{'\''code_type'\'': '\''ICD-10'\'',\n'\''code_value'\'': '\''M10.9'\'',\n'\''code_explain'\'': '\''Gout unspecified'\''},\n{'\''code_type'\'': '\''ICD-10'\'',\n'\''code_value'\'': '\''L98.9'\'',\n'\''code_explain'\'': '\''Purpura, unspecified'\''},\n{'\''code_type'\'': '\''ICD-10'\'',\n'\''code_value'\'': '\''B01.2'\'',\n'\''code_explain'\'': '\''Pseudomonas aeruginosa infection'\''},\n{'\''code_type'\'': '\''ICD-10'\'',\n'\''code_value'\'': '\''J44.1'\'',\n'\''code_explain'\'': '\''Chronic obstructive pulmonary disease'\''},\n{'\''code_type'\'': '\''CPT'\'',\n'\''code_value'\'': '\''0005T'\'',\n'\''code_explain'\'': '\''Chest radiography, single view, frontal'\''},\n{'\''code_type'\'': '\''CPT'\'',\n'\''code_value'\'': '\''95025'\'',\n'\''code_explain'\'': '\''Echocardiography, transthoracic'\''},\n{'\''code_type'\'': '\''CPT'\'',\n'\''code_value'\'': '\''87650'\'',\n'\''code_explain'\'': '\''Skin biopsy, punch'\''},\n{'\''code_type'\'': '\''CPT'\'',\n'\''code_value'\'': '\''87209'\'',\n'\''code_explain'\'': '\''Sputum culture, routine'\''},\n{'\''code_type'\'': '\''CPT'\'',\n'\''code_value'\'': '\''99223-99238'\'',\n'\''code_explain'\'': '\''Office or other outpatient visit for the evaluation and management of an established patient...'\''},\n{'\''code_type'\'': '\''DRG'\'',\n'\''code_value'\'': '\''896'\'',\n'\''code_explain'\'': '\''Cardiac dysrhythmias'\''},\n{'\''code_type'\'': '\''DRG'\'', '\''code_value'\'': '\''483'\'', '\''code_explain'\'': '\''Pneumonia'\''},\n{'\''code_type'\'': '\''DRG'\'',\n'\''code_value'\'': '\''963'\'',\n'\''code_explain'\'': '\''COPD and related conditions'\''},\n{'\''code_type'\'': '\''HCPCS'\'',\n'\''code_value'\'': '\''J7310'\'',\n'\''code_explain'\'': '\''Albuterol sulfate inhalation solution'\''},\n{'\''code_type'\'': '\''HCPCS'\'',\n'\''code_value'\'': '\''J3301'\'',\n'\''code_explain'\'': '\''Ipratropium bromide inhalation solution'\''},\n{'\''code_type'\'': '\''HCPCS'\'',\n'\''code_value'\'': '\''Q4016'\'',\n'\''code_explain'\'': '\''Metformin hydrochloride tablet'\''},\n{'\''code_type'\'': '\''HCPCS'\'',\n'\''code_value'\'': '\''J1800'\'',\n'\''code_explain'\'': '\''Metoprolol tartrate tablet'\''},\n{'\''code_type'\'': '\''HCPCS'\'',\n'\''code_value'\'': '\''J0630'\'',\n'\''code_explain'\'': '\''Amlodipine besylate tablet'\''},\n{'\''code_type'\'': '\''HCPCS'\'',\n'\''code_value'\'': '\''J1349'\'',\n'\''code_explain'\'': '\''Atorvastatin calcium tablet'\''},\n{'\''code_type'\'': '\''HCPCS'\'',\n'\''code_value'\'': '\''J7750'\'',\n'\''code_explain'\'': '\''Ramipril tablet'\''},\n{'\''code_type'\'': '\''HCPCS'\'',\n'\''code_value'\'': '\''J3105'\'',\n'\''code_explain'\'': '\''ASA enteric coated tablet'\''},\n{'\''code_type'\'': '\''HCPCS'\'',\n'\''code_value'\'': '\''J0305'\'',\n'\''code_explain'\'': '\''Citalopram hydrobromide tablet'\''},\n{'\''code_type'\'': '\''HCPCS'\'',\n'\''code_value'\'': '\''J9165'\'',\n'\''code_explain'\'': '\''Terazosin hydrochloride tablet'\''},\n{'\''code_type'\'': '\''HCPCS'\'',\n'\''code_value'\'': '\''Q6315'\'',\n'\''code_explain'\'': '\''Ferrous fumarate tablet'\''}]}";
    
    let user_id = &file_data.user_id;
    let medical_info = &file_data.medical_info;
    
    let prompt = format!(
        "First:  Pass on the entire text to the list_of_medical_terms tool to find the correct medical terms in the discharge report\nSecond: Find the associated medical codes and their short explanation, for these medical terms that you just retrieved,  and use only the below codes:\n1- ICD-10 : International Classification of Diseases, 10th Revision,\n2- CPT Codes:  Current Procedural Terminology,\n3- DRG Codes: Diagnosis Related Groups.\n4- HCPCS Codes: Healthcare Common Procedure Coding System.\nThird:  Format the output as a JSON object with the following keys:\ncode_type\ncode_value\ncode_explain\n\n{}\n\nuse the below as an example of the output:\n\n{}",
        medical_info,
        examples,
    );

    let response = llm
        .with_temperature(0.7)
        .with_top_k(64)
        .with_top_p(0.95)
        .with_max_tokens(16384)
        .with_max_retries(3)
        .with_system_prompt(system_prompt)
        .invoke(&prompt)
        .await?;
        
    let mut result_string = String::new();

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        result_string.push_str(&text);
                    }
                }
            }
        }
    };

    // Remove the ```json and ``` markers and trim whitespace
    let replaced = result_string.replace("```json", "").replace("```", "");
    let json_str = replaced.trim();

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Update MongoDB ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let medical_data: MedicalDummie = match serde_json::from_str(json_str) {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("Error parsing JSON: {}", e);
            return Err(Error::from("Error parsing JSON"));
        }
    };

    // Now you can access the data
    // for term in &medical_data.medical_terms {
    //     println!("Code Type: {}, Value: {}, Explanation: {}", 
    //         term.code_type, term.code_value, term.code_explain);
    // }

    match mongodb_update(
        user_id, 
        medical_info, 
        medical_data,
    ).await {
        Ok(_) => tracing::info!("MongoDB update successful"),
        Err(e) => {
            tracing::error!("MongoDB update failed: {}", e);
            let error_message = format!("MongoDB update failed: {}", e);
            return Err(Error::from(error_message));
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~ Move file to output bucket ~~~~~~~~~~~~~~~~~~~~~~~~~~~

    match move_s3_object(
        &s3_client, 
        &bucket, 
        object_key, 
        &output_bucket
    ).await {
        Ok(_) => tracing::info!(
                "File moved to output bucket. From {}/{} to {}/{}",
                bucket,
                object_key,
                output_bucket,
                object_key
        ),
        Err(e) => {
            tracing::error!("Error moving file to output bucket: {:?}", e);
            return Err(Error::from(e.to_string()));
        }
    }

    Ok(())
}