use lambda_http::{Body, Error, Request, RequestExt, Response};
use lambda_http::tracing::info;
use serde_json::{json, Value};

#[derive(Debug)]
struct Condition {
    code: String,
    display: String,
    status: String,
    onset: String,
}

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("Event received: {:?}", event);
    
    // Extract the path
    let path = event.uri().path();
    
    match path {
        "/Dev/cds-services" => handle_discovery(),
        "/Dev/cds-services/0001" => {
            // Parse the request body
            match event.body() {
                Body::Text(body_str) => {
                    if let Ok(json_data) = serde_json::from_str::<Value>(body_str) {
                        handle_patient_view(json_data)
                    } else {
                        default_response()
                    }
                },
                Body::Binary(bytes) => {
                    if let Ok(body_str) = String::from_utf8(bytes.to_vec()) {
                        if let Ok(json_data) = serde_json::from_str::<Value>(&body_str) {
                            handle_patient_view(json_data)
                        } else {
                            default_response()
                        }
                    } else {
                        default_response()
                    }
                },
                _ => default_response(),
            }
        }
        _ => default_response(),
    }
}

fn handle_discovery() -> Result<Response<Body>, Error> {
    let discovery_response = json!({
        "services": [{
            "hook": "patient-view",
            "title": "Patient Health Review",
            "description": "Analyzes patient conditions and provides relevant alerts",
            "id": "0001",
            "prefetch": {
                "patient": "Patient/{{context.patientId}}",
                "conditions": "Condition?patient={{context.patientId}}"
            }
        }]
    });

    create_response(discovery_response)
}

fn handle_patient_view(data: Value) -> Result<Response<Body>, Error> {
    let mut cards = Vec::new();
    
    // Extract conditions from prefetch data
    if let Some(conditions) = data["prefetch"]["conditions"]["entry"].as_array() {
        let active_conditions: Vec<Condition> = conditions
            .iter()
            .filter_map(|entry| {
                let resource = &entry["resource"];
                let clinical_status = resource["clinicalStatus"]["coding"][0]["code"].as_str()?;
                
                if clinical_status == "active" {
                    Some(Condition {
                        code: resource["code"]["coding"][0]["code"].as_str()?.to_string(),
                        display: resource["code"]["coding"][0]["display"].as_str()?.to_string(),
                        status: clinical_status.to_string(),
                        onset: resource["onsetDateTime"].as_str()?.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        // Generate cards based on conditions
        if !active_conditions.is_empty() {
            cards.push(json!({
                "summary": format!("Patient has {} active conditions", active_conditions.len()),
                "indicator": "warning",
                "detail": format_conditions(&active_conditions),
                "source": {
                    "label": "Condition Analysis Service",
                    "url": "https://example.com/condition-analysis"
                },
                "suggestions": [{
                    "label": "Review all active conditions",
                    "uuid": "review-conditions"
                }]
            }));
        }
    }

    // If no cards were generated, provide a default info card
    if cards.is_empty() {
        cards.push(json!({
            "summary": "No active conditions found",
            "indicator": "info",
            "source": {
                "label": "Condition Analysis Service"
            }
        }));
    }

    create_response(json!({ "cards": cards }))
}

fn format_conditions(conditions: &[Condition]) -> String {
    let mut detail = String::from("Active conditions:\n\n");
    for condition in conditions {
        detail.push_str(&format!(
            "- {} ({})\n  Onset: {}\n",
            condition.display,
            condition.code,
            condition.onset
        ));
    }
    detail
}

fn default_response() -> Result<Response<Body>, Error> {
    create_response(json!({
        "cards": [{
            "summary": "Invalid request",
            "indicator": "warning",
            "source": {
                "label": "CDS Service"
            }
        }]
    }))
}

fn create_response(body: serde_json::Value) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::Text(body.to_string()))
        .map_err(Box::new)?)
}