use aws_lambda_events::{
    event::sqs::{SqsBatchResponse, SqsEventObj, SqsMessageObj},
    sqs::{BatchItemFailure},
};

use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

/// Object that you send to SQS and plan to process on the function.
#[derive(Deserialize, Serialize)]
struct Data {
    id: String,
    text: String,
}

async fn process_record(_: &SqsMessageObj<Data>) -> Result<(), Error> {
    Err(Error::from("Error processing message"))
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<SqsEventObj<Data>>) -> Result<SqsBatchResponse, Error> {
    let mut batch_item_failures = Vec::new();

    for record in event.payload.records {

        let data = &record.body;
        tracing::info!(id = ?data.id, text = ?data.text, "data received from SQS");
                
        match process_record(&record).await {
            Ok(_) => (),
            Err(_) => batch_item_failures.push(BatchItemFailure {
                item_identifier: record.message_id.unwrap(),
            }),
        }
    }

    Ok(SqsBatchResponse {
        batch_item_failures,
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
