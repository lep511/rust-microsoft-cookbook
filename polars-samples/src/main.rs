use polars::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let now = Instant::now();
    println!("reading delays csv dataframe");

    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("fa_articles.csv".into()))
        .expect("can read delays csv")
        .finish()
        .expect("can create delays dataframe");

    println!("done reading csv {:?}", now.elapsed());

    let result = df
        .clone()
        .lazy()
        .with_columns([
            (col("title") + lit(" ") + col("content")).alias("text_for_embedding"),
        ])
        .collect()
        .expect("can collect");
    
    println!("{}", result);

    let mut embeddings_vec = Vec::new();

    // Iterate over the DataFrame
    let text_series = result.column("text_for_embedding").expect("column exists");
    for value in text_series.iter() {
        if let AnyValue::String(text) = value {
            let embedding = embed_text(text).await;
            embeddings_vec.push(embedding);
        }
    }

}

async fn embed_text(text: &str) -> Vec<f64> {
    // mock embedding
    vec![1.0, 2.0, 3.0]
}