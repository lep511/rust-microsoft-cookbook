use mongodb::{ 
    bson::{Document, doc},
    Client, Collection,
};
use crate::utils::{
    NewsDataEmbed, get_embedding,
    format_news_embed, read_json_file,
};
use futures::TryStreamExt;
use std::error::Error;
use serde::Deserialize;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::env;

#[derive(Deserialize, Debug)]
struct VectorData {
    vector: Vec<f32>,
}

pub(crate) async fn handler_basic_query() -> mongodb::error::Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&uri).await?;

    let file_path = "news_check.jsonl";
    let contents = match read_json_file(file_path).await {
        Some(contents) => contents,
        None => {
            println!("Failed to read file");
            return Ok(());
        }
    };

    let line_count = contents.lines().count();
    let mut news_embed: Vec<NewsDataEmbed> = Vec::new();

    for line in contents.lines() {
        match &serde_json::from_str(&line) {
            Ok(data) => {
                let format_field: String = format_news_embed(data);
                let response = match get_embedding(&format_field).await {
                    Ok(embed) => embed,
                    Err(e) => {
                        println!("Failed to get embedding {:?}", e);
                        continue;
                    }
                };
                
                let mut embeddings: Vec<f32> = Vec::new();

                if let Some(em_data) = response.data {
                    for embedding_data in em_data {
                        if let Some(embedding) = embedding_data.embedding {
                            embeddings = embedding;
                        } else {
                            println!("No embedding found");
                            continue;
                        }
                    }
                }

                let embed_data = NewsDataEmbed {
                    link: data.link.clone(),
                    headline: data.headline.clone(),
                    category: data.category.clone(),
                    short_description: data.short_description.clone(),
                    authors: data.authors.clone(),
                    date: data.date.clone(),
                    news_embedding: embeddings.clone(),
                };
                news_embed.push(embed_data);
            }
            Err(_) => {
                println!("Failed to parse line: {}", line);
                continue;
            }
        }
    }

    let field_select = &news_embed[0];
    
    println!("Number of files processed: {}\n", line_count);
    println!("Headline: {}", field_select.headline);
    println!("Category: {}", field_select.category);
    println!("Short Description: {}", field_select.short_description);
    println!("============================================================\n");
    
    let parse_vector = field_select.news_embedding.clone();
    
    let pipeline  = vec! [
        doc! {
            "$vectorSearch": doc! {
            "queryVector": parse_vector,
            "path": "news_embedding",
            "numCandidates": 150,
            "index": "vector_index",
            "limit": 3
        }
        },
        doc! {
            "$project": doc! {
                "_id": 0,
                "headline": 1,
                "short_description": 1,
                "category": 1,
                "score": doc! { "$meta": "vectorSearchScore"
                }
            }
        }
    ];

    let database = client.database("news");
    let collection: Collection<NewsDataEmbed> = database.collection("news_data");
    let mut results = collection.aggregate(pipeline).await?;
    while let Some(result) = results.try_next().await? {
        println!("{:#?}", result);
    }
    Ok(())
}