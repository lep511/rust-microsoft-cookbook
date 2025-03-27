use env_logger::Env;
mod anthropic;
mod utils;
mod create_database;
use create_database::handler_database;
mod vector_index;
use vector_index::create_vector_index;
mod embedding;
use embedding::handler_embedding;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    match handler_database().await {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }

    // match create_vector_index().await {
    //     Ok(_) => (),
    //     Err(e) => println!("Error: {}", e),
    // }

    // match handler_embedding().await {
    //     Ok(_) => (),
    //     Err(e) => println!("Error: {}", e),
    // }
    
}
