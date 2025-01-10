use futures::StreamExt;
use serde_json::{json, Value};

async fn stream_generate_content() {
    let request = json!({
        "contents": [{
            "role": "user",
            "parts": [{
                "text": "How are you doing today?"
            }]
        }]
    });

    let streaming_resp = generative_model.generate_content_stream(request).await;
    
    let mut stream = streaming_resp.stream;
    while let Some(item) = stream.next().await {
        println!("stream chunk: {}", serde_json::to_string(&item).unwrap());
    }

    let response = streaming_resp.response.await;
    println!("aggregated response: {}", serde_json::to_string(&response).unwrap());
}

#[tokio::main]
async fn main() {
    stream_generate_content().await;
}