use futures_util::StreamExt;
use futures_util::TryStreamExt;
use tokio_util::compat::TokioAsyncReadCompatExt;

/// Processes a CSV file asynchronously, reading and printing each record.
///
/// # Arguments
///
/// * `csv_file_path` - Path to the CSV file to process
///
/// # Returns
///
/// Result indicating success or an error that occurred during processing
pub async fn process_csv(
    csv_file_path: &str
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Open the file using tokio's async file operations
    let file = tokio::fs::File::open(csv_file_path).await?;
    
    // Convert the tokio::fs::File to a type that implements futures::AsyncRead
    let compat_file = file.compat();
    
    // Create the CSV reader with the compatible file
    let mut reader = csv_async::AsyncReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .create_reader(compat_file);
    
    // Process each record in the CSV file
    let mut records = reader.records();
    
    loop {
        match records.try_next().await {
            Ok(Some(record)) => {
                // Process the record
                println!("{:?}", record);
            },
            Ok(None) => {
                // End of stream
                break;
            },
            Err(e) => {
                eprintln!("Error processing record: {}", e);
                // Uncomment to fail on first error:
                // return Err(e.into());
            }
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
    let csv_file_path = "dataset.csv";
    
    match process_csv(csv_file_path).await {
        Ok(_) => println!("CSV processing completed successfully"),
        Err(e) => eprintln!("Error processing CSV: {}", e),
    }
}