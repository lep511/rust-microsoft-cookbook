use aws_sdk_s3tables::Client;
mod table_manager;
use table_manager::create_table_from_yaml;
mod utils;
use utils::{create_namespace, get_namespace};

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let table_bucket_arn = "arn:aws:s3tables:us-east-1:491085411627:bucket/my-s3table-49585733";

    let namespace = "flight";

    // Check if namespace exists
    match get_namespace(&client, table_bucket_arn, namespace).await {
        Ok(_) => println!("Namespace exists"),
        Err(_) => {
            match create_namespace(&client, table_bucket_arn, namespace).await {
                Ok(_) => println!("Namespace created successfully"),
                Err(e) => println!("Error creating namespace: {}", e),
            }
        }
    }
    
    match create_table_from_yaml(
        &client, 
        table_bucket_arn, 
        "templates/flight_data.yaml"
    ).await {
        Ok(_) => println!("Table created successfully"),
        Err(e) => println!("Error creating table: {}", e),
    }
    
}