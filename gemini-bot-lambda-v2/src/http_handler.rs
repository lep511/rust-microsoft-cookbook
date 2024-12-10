use lambda_http::{Body, Error, Request, RequestExt, Response};
use gemini_lib::{ LlmResponse, OrderState, generate_content };
use mongodb_lib::{ MongoResponse, mongodb_connect, mongodb_update };

mod gemini_lib;
mod mongodb_lib;
mod bot;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let prompt = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("prompt"))
        .unwrap_or("test2563kd98");

    let user_id = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("user_id"))
        .unwrap_or("None");

    let mongo_result: MongoResponse = match mongodb_connect(&user_id).await {
        Ok(mongo_result) => mongo_result,
        Err(e) => {
            let message = format!("Initial connection to MongoDB fails: {}", e);
            let resp = Response::builder()
                .status(408)
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
                .status(405)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
    };

    let text_parts = llm_result.gemini_response.candidates[0].content.parts[0].text.clone();
    let update_chat = format!("{}\nResponse {}\n\n{}\n", input_text, nc_count, text_parts); 
    println!("{}", update_chat);

    let resp: OrderState = match serde_json::from_str(&text_parts) {
        Ok(resp) => resp,
        Err(e) => {
            let message = format!("Error parsing JSON: {}", e);
            let resp = Response::builder()
                .status(404)
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
                .status(400)
                .header("content-type", "text/html")
                .body(message.into())
                .map_err(Box::new)?;
            return Ok(resp)
        }
    };

    println!("Ok {:?}", mongo_result);

    let message = resp.response.ok_or("Response is missing")?;
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    
    Ok(resp)
}