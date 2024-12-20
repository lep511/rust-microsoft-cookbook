mod gemini_op;

use gemini_op::get_gemini_response;
use lambda_http::{Body, Error, Request, RequestExt, Response};
use std::env;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let prompt = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("prompt"))
        .unwrap_or("None");

    if prompt == "None" {
        let resp = Response::builder()
            .status(400)
            .header("content-type", "text/html")
            .body("Please provide a prompt in string parameters".into())
            .map_err(Box::new)?;
        return Ok(resp)
    }

    let google_api_token = match env::var("GOOGLE_API_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            let resp = Response::builder()
                .status(404)
                .header("content-type", "text/html")
                .body("GOOGLE_API_TOKEN not set".into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    match get_gemini_response(&prompt, google_api_token).await {
        Ok(content) => {
            let message = "Ok";
            let resp = Response::builder()
                .status(200)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
        Err(e) => {
            let message = format!("Error getting response from Gemini - {}", e);
            let resp = Response::builder()
                .status(500)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
    }
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
        query_string_parameters.insert("name".into(), "multimodal_prompts".into());

        let request = Request::default()
            .with_query_string_parameters(query_string_parameters);

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello multimodal_prompts, this is an AWS Lambda HTTP request"
        );
    }
}
