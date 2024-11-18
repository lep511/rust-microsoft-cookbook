mod gemini_op;

use gemini_op:: { LlmResponse, get_gemini_response };
use lambda_http::{Body, Error, Request, RequestExt, Response};

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");

    match get_gemini_response().await {
        Ok(content) => {
            //let message = content.response.ok_or("Response is missing")?;
            let message = "Ok.";
            let resp = Response::builder()
                .status(200)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
        Err(_) => {
            let message = "Error getting response from Gemini";
            let resp = Response::builder()
                .status(500)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
    }
}
