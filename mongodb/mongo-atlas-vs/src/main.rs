#![recursion_limit = "2560"]
mod vector_index;
use vector_index::create_vector_index;
mod basic_query;
use basic_query::handler_basic_query;

#[tokio::main]
async fn main() {
    match create_vector_index().await {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
    
    match handler_basic_query().await {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}
