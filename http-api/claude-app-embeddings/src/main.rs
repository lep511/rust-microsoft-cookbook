use futures::future::try_join_all;
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error as StdError;
use std::time::Duration;
use tokio::time::timeout;

const TIMEOUT_SECONDS: u64 = 30;
const MAX_RETRIES: u32 = 3;
const BATCH_SIZE: usize = 32;

#[derive(Serialize)]
#[allow(dead_code)]
struct EmbeddingsRequest {
    input: Vec<String>,
    model: String,
    input_type: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct EmbeddingsResponse {
    object: String,
    data: Vec<EmbeddingData>,
    model: String,
    usage: Usage,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct EmbeddingData {
    object: String,
    embedding: Vec<f32>,
    index: usize,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Usage {
    total_tokens: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct ProcessedEmbeddings {
    embeddings: Vec<Vec<f32>>,
    total_tokens: u32,
    texts: Vec<String>, // Store original texts for reference
}

#[derive(Debug)]
#[allow(dead_code)]
struct EmbeddingStats {
    min: f32,
    max: f32,
    mean: f32,
    l2_norm: f32,
}

type Result<T> = std::result::Result<T, Box<dyn StdError + Send + Sync>>;

impl ProcessedEmbeddings {

    fn display_all(&self) {
        println!("\nProcessed {} embeddings:", self.embeddings.len());
        println!("Total tokens used: {}", self.total_tokens);
    }

    fn calculate_similarity(&self, index1: usize, index2: usize) -> Option<f32> {
        let embedding1 = self.embeddings.get(index1)?;
        let embedding2 = self.embeddings.get(index2)?;

        if embedding1.len() != embedding2.len() {
            return None;
        }

        let mut dot_product = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;

        for i in 0..embedding1.len() {
            dot_product += (embedding1[i] * embedding2[i]) as f64;
            norm1 += (embedding1[i] * embedding1[i]) as f64;
            norm2 += (embedding2[i] * embedding2[i]) as f64;
        }

        let similarity = dot_product / (norm1.sqrt() * norm2.sqrt());
        Some(similarity as f32)
    }
}

async fn create_embedding_batch(
    client: &Client,
    texts: Vec<String>,
    model: &str,
    api_key: &str,
) -> Result<EmbeddingsResponse> {
    let request_body = EmbeddingsRequest {
        input: texts,
        model: model.to_string(),
        input_type: "document".to_string(),
    };

    for retry in 0..MAX_RETRIES {
        let result = timeout(
            Duration::from_secs(TIMEOUT_SECONDS),
            client
                .post("https://api.voyageai.com/v1/embeddings")
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&request_body)
                .send(),
        )
        .await;

        match result {
            Ok(Ok(response)) => {
                if response.status().is_success() {
                    return Ok(response.json().await?);
                } else if response.status().is_server_error() && retry < MAX_RETRIES - 1 {
                    tokio::time::sleep(Duration::from_millis(100 * 2_u64.pow(retry))).await;
                    continue;
                } else {
                    return Err(format!(
                        "API request failed with status: {}",
                        response.status()
                    )
                    .into());
                }
            }
            Ok(Err(_)) if retry < MAX_RETRIES - 1 => {
                tokio::time::sleep(Duration::from_millis(100 * 2_u64.pow(retry))).await;
                continue;
            }
            Ok(Err(e)) => return Err(Box::new(e)),
            Err(_) if retry < MAX_RETRIES - 1 => {
                tokio::time::sleep(Duration::from_millis(100 * 2_u64.pow(retry))).await;
                continue;
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    Err("Max retries exceeded".into())
}

async fn process_texts(texts: Vec<String>, model: &str) -> Result<ProcessedEmbeddings> {
    let api_key = env::var("VOYAGE_API_KEY")?;

    let client = ClientBuilder::new()
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .tcp_keepalive(Duration::from_secs(60))
        .build()?;

    let batches: Vec<Vec<String>> = texts
        .chunks(BATCH_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect();

    let futures = batches
        .into_iter()
        .map(|batch| create_embedding_batch(&client, batch, model, &api_key));

    let results: Vec<EmbeddingsResponse> = try_join_all(futures).await?;

    let mut all_embeddings = Vec::new();
    let mut total_tokens = 0;

    for response in results {
        total_tokens += response.usage.total_tokens;
        for data in response.data {
            all_embeddings.push(data.embedding);
        }
    }

    Ok(ProcessedEmbeddings {
        embeddings: all_embeddings,
        total_tokens,
        texts,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let reference_text = "The Mediterranean diet emphasizes fish, olive oil, and vegetables, believed to reduce chronic diseases.".to_string();
    
    let texts = vec![
        reference_text.clone(),
        "The Nordic diet highlights whole grains, fatty fish, and root vegetables, thought to lower the risk of heart disease.".to_string(),
        "Photosynthesis in plants converts light energy into glucose and produces essential oxygen.".to_string(),
        "20th-century innovations, from radios to smartphones, centered on electronic advancements.".to_string(),
        "Rivers provide water, irrigation, and habitat for aquatic species, vital for ecosystems.".to_string(),
        "Shakespeare's works, like 'Hamlet' and 'A Midsummer Night's Dream,' endure in literature.".to_string()
    ];

    let start = std::time::Instant::now();
    
    // Models embeddings: https://docs.voyageai.com/docs/embeddings#model-choices
    
    match process_texts(texts, "voyage-3-lite").await {
        Ok(result) => {
            println!("Processing completed in {:?}", start.elapsed());
            
            // Display token count
            result.display_all();

            if result.embeddings.len() >= 2 {
                println!("Comparing similarities with reference text: \"{}\"\n", reference_text);
                for i in 1..result.embeddings.len() {
                    if let Some(similarity) = result.calculate_similarity(0, i) {
                        println!(
                            "Similarity between \"{}\": {:.4}",
                            result.texts[i], similarity
                        )
                    }
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}