use lambda_http::{Body, Request, Error, Response};
use lambda_http::tracing::{error, info};
use crate::libs::manage_hook_data;
use crate::http_page::get_main_page;
use serde_json::json;
use url::Url;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    info!("Event: {:?}", event);
    // info!("Body: {:?}", event.body());
    let api_url = event.uri().to_string();
    let path_fm = event.uri().path();

    if extract_uri_path(&api_url) == "launch" {
        info!("Services path launch");
        let body = get_main_page("no json data");
        return Ok(Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body(body.into())
            .map_err(Box::new)?);
    }
    
    let path: String = path_fm.rsplitn(3, '/')
        .take(2)
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect::<Vec<&str>>()
        .join("/");

    let body_string = match event.body() {
        Body::Text(text) => text,
        _ => ""
    };
    let body_resp: String;

    match path.as_str() {
        "cds-services/0001" => {
            info!("Services path cds-services-0001");
            body_resp = handle_patient_view(&body_string, &api_url).await;
        }
        "cds-services/medication" => {
            info!("Services path cds-services-medication");
            body_resp = handle_patient_view(&body_string, &api_url).await;
        }
        _ => {
            info!("Services path cds-services-0001");
            body_resp = handle_discovery();
        }
    }

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body_resp.into())
        .map_err(Box::new)?)
}

// Function Handle Patient View
pub async fn handle_patient_view(
    hook_data: &str,
    api_url: &str,
) -> String {
    match manage_hook_data(hook_data, api_url).await {
        Ok(body) => body,
        Err(error) => {
            error!("Error: {:?}", error);
            handle_error()
        }
    }
}

// Function Handle Discovery
pub fn handle_discovery() -> String {
    let body = json!({ 
        "services": [
            {
                "hook": "patient-view",
                "title": "Patient Medication",
                "description": "Patient medication description",
                "id": "medication",
                "prefetch": {
                    "patient": "Patient/{{context.patientId}}",
                    "medication": "Medication?patient={{context.patientId}}",
                    "medications_stat": "MedicationStatement?patient={{context.patientId}}",
                    "medication_req": "MedicationRequest?patient={{context.patientId}}"
                }
            }
        ]
    });
    
    body.to_string()
}

fn handle_error() -> String {
       
    let body = json!({ 
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

    body.to_string()
}

fn extract_uri_path(uri: &str) -> String {
    if let Ok(parsed_url) = Url::parse(uri) {
        // Get the path part of the URL
        let path = parsed_url.path();
        
        // Extract just the last path segment
        if let Some(last_segment) = path.split('/').last() {
            if !last_segment.is_empty() {
                println!("Path: /{}", last_segment);
                return last_segment.to_string();
            } else {
                // If the last segment is empty (URL ends with a slash)
                // Get the previous one
                let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
                if let Some(segment) = segments.last() {
                    println!("Path: /{}", segment);
                    return segment.to_string();
                }
            }
        }
    }
    return "".to_string();
}