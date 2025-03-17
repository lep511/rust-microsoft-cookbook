use aws_sdk_s3tables::Client;
mod build;
use build::create_table_from_yaml;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let table_bucket_arn = "arn:aws:s3tables:us-east-1:491085411627:bucket/my-s3table-49585733";
    
    // Use the new function with a path to the YAML template
    match create_table_from_yaml(
        &client, 
        table_bucket_arn, 
        "templates/flight_data.yaml"
    ).await {
        Ok(_) => println!("Table created successfully"),
        Err(e) => println!("Error creating table: {}", e),
    }
    
}