use langchain::gemini::chat::ChatGemini;
use schemars::JsonSchema;
use schemars::schema::RootSchema;
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct MarketingCampaignBrief {
    campaign_name: String,
    campaign_objectives: Vec<String>,
    target_audience: String,
    media_strategy: Vec<String>,
    timeline: String,
    target_countries: Vec<String>,
    performance_metrics: Vec<String>,
}

/// Transforms a JSON schema into a simplified representation
///
/// # Arguments
///
/// * `schema` - A RootSchema instance containing the JSON schema to transform
/// * `sub_struct` - A boolean flag indicating whether to wrap the schema in an array structure
///
/// # Returns
///
/// Returns a Result containing either:
/// * Ok(Value) - A serde_json::Value containing the transformed schema 
/// * Err(Box<dyn Error>) - An error if:
///   - The schema cannot be serialized to JSON
///   - The serialized JSON is not an object
///
pub fn generate_schema(
    schema: RootSchema, 
    sub_struct: bool
) -> Result<Value, Box<dyn std::error::Error>> {
    let response_json = match serde_json::to_value(schema) {
        Ok(value) => value,
        Err(e) => return Err(Box::new(e)),
    };

    let mut response = match response_json.as_object() {
        Some(obj) => obj.clone(),
        None => return Err("Serialized JSON is not an object".into()),
    };

    response.remove("$schema");
    response.remove("title");
    response.remove("definitions");

    if sub_struct {
        response.remove("required");
        return Ok(json!({
            "type": "array",
            "items": response
        }))
    }

    Ok(Value::Object(response))
}

async fn example_tools() -> Result<(), Box<dyn std::error::Error>> {

    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;
    let file_path = Some("tests/files/sample_marketing_campaign.pdf");
    let upload_data = None;
    let display_name = "campaign.pdf";
    let mime_type = "auto";

    // Generate schema
    let schema = schemars::schema_for!(MarketingCampaignBrief);
    let json_schema = generate_schema(schema ,false)?;

    let prompt = "Extract the details from the sample marketing brief.";

    let response = llm
        .with_json_schema(json_schema)
        .media_upload(
            file_path,
            upload_data,
            display_name,
            mime_type,
        )
        .await?
        .invoke(prompt)
        .await?;
    
    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                    }
                }
            }
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    example_tools().await?;
    Ok(())
}