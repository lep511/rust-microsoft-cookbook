use lambda_http::{Body, Error, Request, RequestExt, Response};
use lambda_http::tracing::info;
use serde_json::{Value, json};

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    info!("Event: {:?}", event);
    let path_fm = event.uri().path();
    let method = event.method();

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
            handle_patient_view()
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

fn handle_patient_view() -> Result<Response<Body>, Error> {
    let patient_view = json!({ 
        "cards": [
            {
                "summary": "patient-view",
                "indicator": "info",
                "source": {
                    "label": "test service"
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
