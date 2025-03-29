use mongodb::{Client, error::Result};
use mongodb::bson::{doc, Document};
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    plot: String,
    title: String,
}

pub(crate) async fn handler_adv_query() -> Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&uri).await?;
    
    // Access the database and collection
    let database = client.database("sample_mflix");
    let collection = database.collection::<Movie>("movies");

    let query_str = "ana";
    
    // Create the search pipeline
    let pipeline = vec![
        doc! {
            "$search": {
                "text": {
                    "query": "jungle",
                    "path": "plot",
                },
            },
        },
        doc! {
            "$limit": 5,
        },
        doc! {
            "$project": {
                "_id": 0,
                "title": 1,
                "plot": 1,
            },
        },
    ];
    
    // Execute the aggregation pipeline
    let mut results = collection.aggregate(pipeline).await?;

    // Process the results
    while let Some(result) = results.try_next().await? {
        // Convert BSON document to a Movie struct
        match mongodb::bson::from_document::<Movie>(result) {
            Ok(movie) => {
                println!("Title: {}  |  Plot: {}", 
                movie.title, movie.plot);
            },
            Err(e) => {
                println!("Error deserializing document: {}", e);
            }
        }
    }

    Ok(())
}