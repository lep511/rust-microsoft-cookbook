use env_logger::Env;
mod basic_query;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    match basic_query::handler_basic_query().await {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
    
}
