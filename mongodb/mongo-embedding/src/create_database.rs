use std::ops::Index;
use futures::{TryStreamExt};
use mongodb::{bson::{Document, doc}, Client, Collection, SearchIndexModel};
use crate::utils::{
    NewsData, NewsDataEmbed, get_embedding,
    format_news_embed, read_json_file,
};
use std::env;

pub(crate) async fn handler_database() -> mongodb::error::Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(&uri).await?;

    // Get a handle on the movies collection
    let database = client.database("news");
    let collection: Collection<Document> = database.collection("news_data");

    let contents = match read_json_file("news_dataset.json").await {
        Some(contents) => contents,
        None => {
            println!("Failed to read file");
            return Ok(());
        }
    };

    let mut news = Vec::new();
    let mut news_embed = Vec::new();

    for line in contents.lines() {
        let data: NewsData = match serde_json::from_str(&line) {
            Ok(data) => {
                let format_field: String = format_news_embed(data);
                let embed = match get_embedding(format_field).await {
                    Ok(embed) => embed,
                    Err(e) => {
                        println!("Failed to get embedding {:?}", e);
                        continue;
                    }
                };
                let embed_data = NewsDataEmbed {
                    title: data.title,
                    description: data.description,
                    content: data.content,
                    url: data.url,
                    image_url: data.image_url,
                    published_at: data.published_at,
                    source: data.source,
                    category: data.category,
                    language: data.language,
                    country: data.country,
                    embed: embed,
                };
                news_embed.push(embed_data);
            }
            Err(_) => {
                println!("Failed to parse line: {}", line);
                continue;
            }
        };
        news.push(data);
    }

    let insert_news_result = collection.insert_many(&news_embed).await?;   
    println!("Total of documents inserted: {}", insert_news_result.inserted_ids.len());

    Ok(())
}

