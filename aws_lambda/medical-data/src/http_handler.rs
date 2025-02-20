use lambda_http::{Body, Error, Request, RequestExt, Response};
use lambda_http::tracing::info;
use serde_json::json;

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

    info!("Path: {:?}", path);

    let mut response_data = json!({ 
        "services": [
            {
                "hook": "patient-view",
                "title": "Patient View",
                "description": "Patient view description",
                "id": "0001"
            }
        ]
    });

    match path.as_str() {
        "cds-services/0001" => {
            info!("Services path cds-services-0001");
            response_data = json!({ 
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
        }
        _ => {
            info!("Default path");
        }
    }

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::Text(response_data.to_string())) // Convert JSON to string
        .map_err(Box::new)?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use lambda_http::{Request, RequestExt};

    #[tokio::test]
    async fn test_generic_http_handler() {
        let request = Request::default();

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello world, this is an AWS Lambda HTTP request"
        );
    }

    #[tokio::test]
    async fn test_http_handler_with_query_string() {
        let mut query_string_parameters: HashMap<String, String> = HashMap::new();
        query_string_parameters.insert("name".into(), "medical-data".into());

        let request = Request::default()
            .with_query_string_parameters(query_string_parameters);

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello medical-data, this is an AWS Lambda HTTP request"
        );
    }
}
