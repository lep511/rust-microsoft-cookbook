use lambda_http::{run, service_fn, tracing, Error};
mod http_handler;
mod libs;
mod gemini;
mod llm_engine;
mod scrab_errors;
mod http_page;
use http_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}