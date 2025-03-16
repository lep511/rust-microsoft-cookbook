use std::collections::HashMap;
use std::env;
use std::time::Duration;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error as DynamoError};
use aws_sdk_dynamodb::types::AttributeValue;
use tokio::time::sleep;

// Custom error type
#[derive(Debug)]
enum AppError {
    DynamoError(DynamoError),
    ProcessingFailed(String),
}

impl From<DynamoError> for AppError {
    fn from(err: DynamoError) -> Self {
        AppError::DynamoError(err)
    }
}

type Result<T> = std::result::Result<T, AppError>;

// Struct to represent task data
#[derive(Debug, Clone)]
struct TaskResult {
    id: usize,
    status: String,
    processing_time_ms: u64,
    created_at: String,
}

// Function to save a task result to DynamoDB
async fn save_to_dynamodb(client: &Client, table_name: &str, result: &TaskResult) -> Result<()> {
    // Create timestamp for the record
    let now = chrono::Utc::now().to_rfc3339();
    
    // Convert our data to DynamoDB format
    let mut item = HashMap::new();
    item.insert("task_id".to_string(), AttributeValue::N(result.id.to_string()));
    item.insert("status".to_string(), AttributeValue::S(result.status.clone()));
    item.insert("processing_time".to_string(), AttributeValue::N(result.processing_time_ms.to_string()));
    item.insert("created_at".to_string(), AttributeValue::S(result.created_at.clone()));
    item.insert("timestamp".to_string(), AttributeValue::S(now));

    // Save to DynamoDB with retries
    let mut retries = 3;
    while retries > 0 {
        match client.put_item()
            .table_name(table_name)
            .set_item(Some(item.clone()))
            .send()
            .await
        {
            Ok(_) => return Ok(()),
            Err(err) => {
                if retries > 1 {
                    println!("DynamoDB error, retrying: {}", err);
                    sleep(Duration::from_millis(200)).await;
                    retries -= 1;
                } else {
                    return Err(AppError::DynamoError(err.into()));
                }
            }
        }
    }
    
    unreachable!()
}

// Process data function with real implementation
async fn process_data(id: usize) -> Result<TaskResult> {
    // Replace this with actual data processing logic
    
    Ok(TaskResult {
        id,
        status: "completed".to_string(),
        processing_time_ms: 0, // Set actual processing time if measured
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up AWS SDK
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    let table_name = env::var("DYNAMODB_TABLE_NAME").expect("DYNAMODB_TABLE_NAME must be set");
    
    println!("Initialized AWS DynamoDB client");

    // Ensure the table exists
    match client.describe_table().table_name(&table_name).send().await {
        Ok(_) => println!("DynamoDB table '{}' exists", table_name),
        Err(e) => {
            eprintln!("Error accessing DynamoDB table: {}", e);
            return Err(AppError::DynamoError(e.into()));
        }
    }
    
    // Process data and save results
    let task_count = 10; // Number of tasks to process
    let mut handles = Vec::with_capacity(task_count);
    
    for id in 0..task_count {
        let client = client.clone();
        let table_name = table_name.clone();
        
        // Process each task concurrently
        let handle = tokio::spawn(async move {
            println!("Processing task {id}");
            
            match process_data(id).await {
                Ok(result) => {
                    if let Err(e) = save_to_dynamodb(&client, &table_name, &result).await {
                        eprintln!("Failed to save task {id} to DynamoDB: {:?}", e);
                        return (id, false);
                    }
                    println!("Task {id} saved to DynamoDB successfully");
                    (id, true)
                },
                Err(e) => {
                    eprintln!("Task {id} processing failed: {:?}", e);
                    (id, false)
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    let mut success_count = 0;
    let mut error_count = 0;
    
    for handle in handles {
        match handle.await {
            Ok((_, success)) => {
                if success {
                    success_count += 1;
                } else {
                    error_count += 1;
                }
            },
            Err(e) => {
                eprintln!("Task join error: {e}");
                error_count += 1;
            }
        }
    }
    
    // Print summary
    println!("\nFinal results summary:");
    println!("Total tasks processed: {}", task_count);
    println!("Success: {}", success_count);
    println!("Errors: {}", error_count);
    
    Ok(())
}