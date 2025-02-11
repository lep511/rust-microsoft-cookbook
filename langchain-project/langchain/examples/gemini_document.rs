#[allow(dead_code)]
use langchain::gemini::chat::ChatGemini;
use env_logger::Env;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use langchain::gemini::utils::generate_schema;
use schemars::JsonSchema;

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct Invoice {
    pub invoice_id: String,
    pub issue_date: String,
    pub items: Vec<Item>,
    pub due_date: String,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
pub struct Item {
    pub description: String,
    pub quantity: i32,    // Don't use u32
    pub unit_price: f64,
    pub amount: f64,
    pub vat: i32,         // Don't use u32
}

async fn small_documents() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let prompt = "What's the document type? Reply from the following options: \
        invoice, Bank Statement, Paystub, Form 1040, Form W-9, Form 1099-R.";

    let file_path = Some("tests/files/w9.pdf");
    let upload_data = None;

    let display_name = "w9.pdf";
    let mime_type = "application/pdf";
    
    let start = Instant::now();
    
    let response = llm
        .media_upload(
            file_path,
            upload_data,
            display_name,
            mime_type,
        )
        .await?
        .invoke(prompt)
        .await?;

    let elapsed = start.elapsed().as_secs_f64();

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                    }
                }
            }
        }
    };

    println!("[Task took {:.2} seconds]", elapsed);
    
    Ok(())
}

async fn large_documents() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let prompt = "Summarize the content of the document for a second year student.";
    let file_path = Some("tests/files/large-document.pdf");
    let upload_data = None;
    let display_name = "dosto-kafka.pdf";
    let mime_type = "application/pdf";

    let start = Instant::now();
    
    let response = llm
        .media_upload(
            file_path,
            upload_data,
            display_name,
            mime_type,
        )
        .await?
        .invoke(prompt)
        .await?;

    let elapsed = start.elapsed().as_secs_f64();

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                    }
                }
            }
        }
    };

    println!("[Task took {:.2} seconds]", elapsed);
    
    Ok(())
}

async fn invoice_documents() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatGemini::new("gemini-2.0-flash-exp")?;

    let prompt = "Extract the structured data from the following PDF file";

    let file_path = Some("tests/files/invoice.pdf");
    // let file_path = Some("tests/files/invoice.png");
    let upload_data = None;
    let display_name = "invoice-document";
    let mime_type = "auto";

    // Generate schema for Item
    let schema_item = schemars::schema_for!(Item);
    let json_schema_item = generate_schema(schema_item, true)?;

    // Generate schema for Invoice
    let schema_invoice = schemars::schema_for!(Invoice);
    let mut json_schema_inv = generate_schema(schema_invoice, false)?;
    json_schema_inv["properties"]["items"] = json_schema_item;

    let response = llm
        .with_json_schema(json_schema_inv)
        .media_upload(
            file_path,
            upload_data,
            display_name,
            mime_type,
        )
        .await?
        .invoke(prompt)
        .await?;

    if let Some(candidates) = response.candidates {
        for candidate in candidates {
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        println!("{}", text);
                    }
                }
            }
        }
    };

    Ok(())
}
    
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // small_documents().await?;
    // large_documents().await?;
    invoice_documents().await?;  
    
    Ok(())
}