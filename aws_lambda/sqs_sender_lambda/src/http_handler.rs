use lambda_http::{Body, Error, Request, RequestExt, Response};
use aws_sdk_sqs::{Client, Error as SqsError};
use std::env;

async fn send_message(
    client: &Client,
    queue_url: &str,
    message_body: &str,
) -> Result<(), SqsError> {
    client
        .send_message()
        .queue_url(queue_url)
        .message_body(message_body)
        .send()
        .await?;
    
    Ok(())
}

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");
    
    let queue_url = match env::var("SQS_QUEUE_URL") {
        Ok(val) => val,
        Err(_) => {
            let resp = Response::builder()
                .status(500)
                .header("content-type", "text/html")
                .body("SQS_QUEUE_URL not found".into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    // Send the message to SQS
    send_message(&client, &queue_url, &message).await?;

    let body_message = format!("Message sent to SQS: {}", message);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(body_message.into())
        .map_err(Box::new)?;
    Ok(resp)
}