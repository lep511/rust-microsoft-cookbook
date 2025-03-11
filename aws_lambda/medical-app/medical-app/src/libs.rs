use serde_json::{json, Value, from_str};
use lambda_http::tracing::{error, warn, info};
use crate::llm_engine::manage_medication;
use crate::scrab_errors::ScrabError;
use url::Url;

fn extract_medications_stat(json_str: &str) -> Vec<String> {
    // Parse the JSON string into a serde_json::Value
    let v: Value = match from_str(json_str) {
        Ok(value) => value,
        Err(e) => {
            warn!("Failed to parse JSON: {}", e);
            return vec![];
        }
    };

    // Navigate to the "prefetch.medications_stat.entry" array
    let entries = match v.get("prefetch")
        .and_then(|p| p.get("medications_stat"))
        .and_then(|m| m.get("entry"))
        .and_then(|e| e.as_array()) {
        Some(array) => array,
        None => {
            warn!("No medications entry array found");
            return vec![];
        }
    };

    // Extract details from each MedicationStatement resource
    let medications: Vec<String> = entries.iter().filter_map(|entry| {
        let resource = entry.get("resource")?;
        
        let name = resource.get("medicationCodeableConcept")
            .and_then(|mc| mc.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("Unknown");

        let status = resource.get("status")
            .and_then(|s| s.as_str())
            .unwrap_or("Unknown");

        let effective = if let Some(date_time) = resource.get("effectiveDateTime").and_then(|dt| dt.as_str()) {
            date_time.to_string()
        } else if let Some(period) = resource.get("effectivePeriod").and_then(|p| p.as_object()) {
            let start = period.get("start").and_then(|s| s.as_str()).unwrap_or("Unknown");
            let end = period.get("end").and_then(|e| e.as_str()).unwrap_or("Unknown");
            format!("from {} to {}", start, end)
        } else {
            "Unknown".to_string()
        };

        Some(format!("{} (status: {}, effective: {})", name, status, effective))
    }).collect();

    medications
}

pub async fn manage_hook_data(
    hook_data: &str,
    api_url: &str,
) -> Result<String, ScrabError> {

    let mut base_url = String::new();

    if let Ok(parsed_url) = Url::parse(api_url) {
        // Reconstruct the base URL up to the /v1/ part
        let path_segments: Vec<&str> = parsed_url.path().split('/').collect();
        if path_segments.len() >= 2 {
            // Build the base URL
            base_url = parsed_url.origin().ascii_serialization();
            base_url.push_str("/");
            base_url.push_str(path_segments[1]);
        }
    }

    let smart_app_uri = format!("{}/launch", base_url);
    info!("Smart App URI: {}", smart_app_uri);

    let medications_stat = extract_medications_stat(hook_data);
    let mut medications_result = String::new();
    let mut response: String;
    
    for med in medications_stat {
        medications_result.push_str(&med);
        medications_result.push_str("\n");
    }

    if !medications_result.is_empty() {
        response = match manage_medication(&medications_result).await {
            Ok(result) => result,
            Err(e) => {
                error!("Error managing medications: {:?}", e);
                "The AI model does not want to work. Try again later. ".to_string()
            }
        };
    } else {
        info!("No medications found");
        response = "No medications were found that the patient is currently taking. ".to_string();
    }

    response.push_str("\n\nClick the button if you want a more detailed report of the patient's medications.");

    let body = json!({ 
        "cards": [
            {
                "summary": "Medication",
                "indicator": "info",
                "source": {
                    "label": "test service"
                },
                "detail": &response,
                "links": [
                    {
                        "label": "MediCompass App",
                        "url": &smart_app_uri,
                        "type": "smart"
                    }
                ]
            }
        ]
    });

    Ok(body.to_string())
}