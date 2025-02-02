use lambda_runtime::{run, service_fn, tracing, Error};
use env_logger::Env;

pub mod mongodb;
pub mod gemini;
pub mod llmerror;
pub mod libs;
mod event_handler;
use event_handler::function_handler;


#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    run(service_fn(function_handler)).await
}
