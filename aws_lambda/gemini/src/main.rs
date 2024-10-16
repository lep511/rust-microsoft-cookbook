mod llm_multimodal;
mod llm_simple;
mod llm_json;

use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use llm_simple::invoke_simple_llm;
use llm_multimodal::invoke_mm_llm;
use llm_json::invoke_json_llm;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let prompt = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("prompt"))
        .unwrap_or("Default prompt");
    
    let message = format!("Prompt: {prompt}");

    match invoke_json_llm().await {
        Ok(_) => println!("LLM invocation successful"),
        Err(e) => eprintln!("Error invoking LLM: {}", e),
    }

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
