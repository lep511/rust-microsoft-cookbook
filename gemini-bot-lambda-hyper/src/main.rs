mod gemini;
mod mongodb;
mod bot;

use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use lambda_http::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use gemini:: { OrderState, Order, Modifier };
use mongodb::mongodb_connect;
use gemini::generate_content;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request    
    let prompt = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("prompt"))
        .unwrap_or("None");

    if prompt == "None" {
        let message = "No prompt text.".to_string();
        let resp = Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("content-type", "text/html")
            .body(message.into())
            .map_err(Box::new)?;
        return Ok(resp)
    }

    let user_id = "idm39403kd98".to_string();

    let mongo_result = match mongodb_connect(user_id).await {
        Ok(mongo_result) => mongo_result,
        Err(e) => {
            let message = format!("Error in MongoDB: {}", e);
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
    };

    match generate_content(&prompt, &mongo_result).await {
        Ok(resp) => {
            let message = resp.response.ok_or("Response is missing")?;
            let resp = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
        Err(e) => {
            let message = format!("Error generating content: {}", e);
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
