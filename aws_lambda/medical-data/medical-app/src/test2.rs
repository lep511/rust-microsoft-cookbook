use lambda_http::{Body, Error, Request, Response};
use lambda_http::tracing::{error, info};
use crate::libs::CDSHooksResponse;
use serde_json::json;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    // info!("Event: {:?}", event);
    info!("Body: {:?}", event.body());
    let path_fm = event.uri().path();
    // let method = event.method();

    let path: String = path_fm.rsplitn(3, '/')
        .take(2)
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect::<Vec<&str>>()
        .join("/");

    match path.as_str() {
        "cds-services/0001" => {
            info!("Services path cds-services-0001");
            handle_patient_view(event.body())
        }
        _ => {
            handle_discovery()
        }
    }
}

fn handle_discovery() -> Result<Response<Body>, Error> {
    let discovery_response = json!({ 
        "services": [
            {
                "hook": "patient-view",
                "title": "Patient View",
                "description": "Patient view description",
                "id": "0001",
                "prefetch": {
                    "patient": "Patient/{{context.patientId}}",
                    "conditions": "Condition?patient={{context.patientId}}"
                }
            }
        ]
    });

    create_response(discovery_response)
}

fn handle_patient_view(hook_data: &Body) -> Result<Response<Body>, Error> {
    
    let body_str = match hook_data {
        Body::Text(body) => body,
        _ => {
            error!("[E0993] Body is not text.");
            return handle_error();
        }
    };

    let cds_response: CDSHooksResponse = match serde_json::from_str(body_str) {
        Ok(data) => data,
        Err(error) => {
            error!("[E0994] Error parsing body: {:?}", error);
            return handle_error();
        }
    };

    let names: (String, String) = match extract_patient_name(&cds_response) {
        Some(names) => names,
        None => {
            error!("[E0995] Error extracting patient name.");
            return handle_error();
        }
    };

    let greeting = format!(
        "Hello, {} {}!", 
        names.0,
        names.1
    );

    let patient_view = json!({ 
        "cards": [
            {
                "summary": greeting,
                "indicator": "info",
                "source": {
                    "label": "test service"
                },
                "links": [
                    {
                        "label": "My App",
                        "url": "https://7tyg9r9mt8.execute-api.us-east-1.amazonaws.com/Dev/launch",
                        "type": "smart"
                    }
                ]
            }
        ]
    });

    create_response(patient_view)
}

fn extract_patient_name(response: &CDSHooksResponse) -> Option<(String, String)> {
    if let Some(prefetch) = &response.prefetch {
        if let Some(patient) = &prefetch.patient {
            if let Some(names) = &patient.name {
                // Assuming the first name in the list is the primary name
                if let Some(human_name) = names.get(0) {
                    let family_name = human_name.family.clone().unwrap_or_default();
                    let given_names = human_name.given.clone().unwrap_or_default();
                    let given_name = given_names.join(" ");
                    return Some((given_name, family_name));
                }
            }
        }
    }
    None
}

fn handle_error() -> Result<Response<Body>, Error> {
       
    let patient_view = json!({ 
        "cards": [
            {
                "summary": "patient-view",
                "indicator": "info",
                "source": {
                    "label": "No event"
                }
            }
        ]
    });

    create_response(patient_view)
}

fn create_response(body: serde_json::Value) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::Text(body.to_string()))
        .map_err(Box::new)?)
}