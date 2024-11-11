use reqwest::{ Client, Body };
use serde_json:: { json, Value };
use std::error::Error;
use std::env;
use std::fs;
use base64::{Engine as _, engine::general_purpose};
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get COHERE key from environment variable
    let api_key = env::var("COHERE_API_KEY")
        .expect("COHERE_API_KEY environment variable is not set");

    let api_key = format!("bearer {}", api_key);

    let texts = vec![
        "When are you open?",
        "When do you close?",
        "What are the hours?",
        "Are you open on weekends?",
        "Are you available on holidays?",
        "How much is a burger?",
        "What's the price of a meal?",
        "How much for a few burgers?",
        "Do you have a vegan option?",
        "Do you have vegetarian?",
        "Do you serve non-meat alternatives?",
        "Do you have milkshakes?",
        "Milkshake",
        "Do you have desert?",
        "Can I bring my child?",
        "Are you kid friendly?",
        "Do you have booster seats?",
        "Do you do delivery?",
        "Is there takeout?",
        "Do you deliver?",
        "Can I have it delivered?",
        "Can you bring it to me?",
        "Do you have space for a party?",
        "Can you accommodate large groups?",
        "Can I book a party here?"
    ];

    // embedding_types:
    //     "float": Use this when you want to get back the default float embeddings.
    //     "int8": Use this when you want to get back signed int8 embeddings.
    //     "uint8": Use this when you want to get back unsigned int8 embeddings.
    //     "binary": Use this when you want to get back signed binary embeddings.
    //     "ubinary": Use this when you want to get back unsigned binary embeddings.
    // truncate: NONE, START or END

    let content = format!(
        r#"{{
            "model": "embed-english-light-v3.0",
            "texts": {},
            "input_type": "classification",
            "embedding_types": ["float"],
            "truncate": "START"
        }}"#, 
        serde_json::to_string(&texts)?
    );

    let body = Body::wrap(content);

        // Create a reqwest client
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;  

    // Send the POST request
    let response = client
        .post("https://api.cohere.com/v2/embed")
        .header("Authorization", &api_key)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    let result: Value = response.json().await?;
    // println!("{:#?}", result);

    // Extract and print only billed units and response type
    if let Some(meta) = result.get("meta") {
        if let Some(billed_units) = meta.get("billed_units") {
            println!("Billed Units: {}", billed_units);
        }
    }
    
    if let Some(response_type) = result.get("response_type") {
        println!("Response Type: {}", response_type);
    }

    // Save formatted (pretty) JSON
    let formatted_json = serde_json::to_string_pretty(&result)?;
    fs::write("output.json", formatted_json)?;


    // Read the JPG file into a buffer
    let mut file = fs::File::open("image.jpeg")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    // Convert to base64
    let base64_string = general_purpose::STANDARD.encode(&buffer);
    
    // Add data URI prefix for HTML/web usage
    let data_uri = format!("data:image/jpeg;base64,{}", base64_string);

    // Create the request body
    let body = json!({
        "model": "embed-english-v3.0",
        "input_type": "image",
        "embedding_types": ["float"],
        "images": [
            data_uri
        ]
    });

    // Send the POST request
    let response = client
        .post("https://api.cohere.com/v2/embed")
        .header("Authorization", &api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let result: Value = response.json().await?;

    // Save formatted (pretty) JSON
    let formatted_json = serde_json::to_string_pretty(&result)?;
    fs::write("output_image.json", formatted_json)?;

    Ok(())

}
