use env_logger::Env;
mod anthropic;
mod utils;
mod create_database;
mod vector_index;
mod embedding;
mod basic_query;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // match create_database::handler_database("news_dataset_3.jsonl").await {
    //     Ok(_) => (),
    //     Err(e) => println!("Error: {}", e),
    // }

    // match vector_index::create_vector_index().await {
    //     Ok(_) => (),
    //     Err(e) => println!("Error: {}", e),
    // }

    // match embedding::handler_embedding().await {
    //     Ok(_) => (),
    //     Err(e) => println!("Error: {}", e),
    // }

    match basic_query::handler_basic_query().await {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
    
}
