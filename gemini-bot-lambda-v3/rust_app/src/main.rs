use lambda_runtime::{run, service_fn, tracing, Error};

pub mod mongodb;
pub mod gemini;
pub mod llmerror;
pub mod libs;
mod event_handler;
use event_handler::function_handler;


#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
