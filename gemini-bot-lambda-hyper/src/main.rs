mod gemini;
mod mongodb;
mod bot;

use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use lambda_http::http::StatusCode;
// use serde::{Deserialize, Serialize};
// use serde_json::json;
use gemini::{ LlmResponse, OrderState, generate_content };
use mongodb::{ MongoResponse, mongodb_connect, mongodb_update };

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

    let user_id = "idm2563kd98".to_string();

    let mongo_result: MongoResponse = match mongodb_connect(&user_id).await {
        Ok(mongo_result) => mongo_result,
        Err(e) => {
            let message = format!("Initial connection to MongoDB fails: {}", e);
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
    };

    // let input_text = mongo_result.user_data;
    let nc_count = mongo_result.chat_count;
    let input_text = format!(
        "{}Input {}\nCustomer: {}",
        mongo_result.user_data,
        nc_count,
        prompt
    );

    let llm_result: LlmResponse = match generate_content(&input_text).await {
        Ok(llm_result) => llm_result,
        Err(e) => {
            let message = format!("Error generating content: {}", e);
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
    };

    let text_parts = llm_result.gemini_response.candidates[0].content.parts[0].text.clone();

    let update_chat = format!("{}\nResponse {}\n\n{}", input_text, nc_count, text_parts);
    
    println!("{}", update_chat);

    let resp: OrderState = match serde_json::from_str(&text_parts) {
        Ok(resp) => resp,
        Err(e) => {
            let message = format!("Error parsing JSON: {}", e);
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
    };

    let mongo_result: MongoResponse = match mongodb_update(&user_id, &update_chat, nc_count).await {
        Ok(mongo_result) => mongo_result,
        Err(e) => {
            let message = format!("Error when updating data in MongoDB: {}", e);
            let resp = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
    };

    println!("Ok {:?}", mongo_result);

    let message = resp.response.ok_or("Response is missing")?;
    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    
    Ok(resp)

    // match generate_content(&prompt, &mongo_result).await {
    //     Ok(resp) => {
    //         let message = resp.response.ok_or("Response is missing")?;
    //         let resp = Response::builder()
    //             .status(StatusCode::OK)
    //             .header("content-type", "text/html")
    //             .body(message.into())
    //             .map_err(Box::new)?;
    //         return Ok(resp)
    //     }
    //     Err(e) => {
    //         let message = format!("Error generating content: {}", e);
    //         let resp = Response::builder()
    //             .status(StatusCode::BAD_REQUEST)
    //             .header("content-type", "text/html")
    //             .body(message.into())
    //             .map_err(Box::new)?;
    //         return Ok(resp)
    //     }
    // }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
