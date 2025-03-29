#![recursion_limit = "2560"]
mod vector_index;
mod basic_query;
mod adv_query;

#[tokio::main]
async fn main() {
    // match vector_index::create_vector_index().await {
    //     Ok(_) => (),
    //     Err(e) => println!("Error: {}", e),
    // }
    
    // match basic_query::handler_basic_query().await {
    //     Ok(_) => (),
    //     Err(e) => println!("Error: {}", e),
    // }

    match adv_query::handler_adv_query().await {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}
