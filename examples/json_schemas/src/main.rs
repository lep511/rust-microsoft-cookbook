use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

fn main() {
    // Genera el schema
    let schema = schemars::schema_for!(MarketingCampaignBrief);
    
    // Convierte el schema a Value
    let response_json = serde_json::to_value(schema.clone()).unwrap();

    let mut response = response_json.as_object().unwrap().clone();
    response.remove("$schema");
    response.remove("title");

    let mut json_schema = serde_json::Value::Object(response);

    json_schema["campaign_objectives"] = json!({"type": "array", "items": {"type": "string"}});
    // Show pretty json
    println!("{}", serde_json::to_string_pretty(&json_schema).unwrap());
}