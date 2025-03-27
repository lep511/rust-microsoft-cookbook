use mongodb::{Client, Collection};
use crate::utils::{
    NewsDataEmbed, get_embedding,
    format_news_embed, read_json_file,
};
use std::env;

pub(crate) async fn handler_database(
    file_path: &str
) -> mongodb::error::Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(&uri).await?;

    // Get a handle on the movies collection
    let database = client.database("news");
    let collection: Collection<NewsDataEmbed> = database.collection("news_data");

    let contents = match read_json_file(file_path).await {
        Some(contents) => contents,
        None => {
            println!("Failed to read file");
            return Ok(());
        }
    };

    let line_count = contents.lines().count();
    let mut news_embed = Vec::new();
    let mut count = 0;

    for line in contents.lines() {
        match &serde_json::from_str(&line) {
            Ok(data) => {
                let format_field: String = format_news_embed(data);
                let response = match get_embedding(&format_field).await {
                    Ok(embed) => {
                        println!("Progress: {} of {}", count, line_count);
                        count += 1;
                        embed
                    }
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

    let insert_news_result = collection.insert_many(&news_embed).await?;   
    println!("Total of documents inserted: {}", insert_news_result.inserted_ids.len());

    Ok(())
}

