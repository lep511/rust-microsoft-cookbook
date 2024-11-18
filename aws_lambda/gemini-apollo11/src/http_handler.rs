mod gemini_op;

use gemini_op::get_gemini_response;
use lambda_http::{Body, Error, Request, RequestExt, Response};

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

    match get_gemini_response(&prompt).await {
        Ok(content) => {
            let message = content.gemini_response.candidates[0].content.parts[0].text.clone();
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
