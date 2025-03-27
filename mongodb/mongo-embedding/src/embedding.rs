use mongodb::{bson::{doc, Document}, Collection, Client};
// use serde::{ Deserialize, Serialize };
use futures::TryStreamExt;
use crate::utils::get_embedding;
use crate::anthropic::libs::EmbedResponse;
use std::env;

pub(crate) async fn handler_embedding() -> mongodb::error::Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(&uri).await?;

    let text_embed = "In 1985, teenager Marty McFly lives in Hill Valley, California, with \
                    his depressed alcoholic mother, Lorraine; his older siblings, who are \
                    professional and social failures; and his meek father, George, who is \
                    bullied by his supervisor, Biff Tannen. After Marty's band fails a music \
                    audition, he confides in his girlfriend, Jennifer Parker, that he fears \
                    becoming like his parents despite his ambitions.";

    let response: EmbedResponse = get_embedding(text_embed)
        .await.expect("Failed to get embedding");

    let mut embeddings: Vec<f32> = Vec::new();

    if let Some(data) = response.data {
        for embedding_data in data {
            if let Some(embedding) = embedding_data.embedding {
                embeddings = embedding;
            } else {
                println!("No embedding found");
                return Ok(());
            }
        }
    }

    let pipeline  = vec! [
        doc! {
            "$vectorSearch": doc! {
            "queryVector": embeddings,
            "path": "plot_embedding",
            "numCandidates": 150,
            "index": "vector_index",
            "limit": 5
        }
        },
        doc! {
            "$project": doc! {
                "_id": 0,
                "plot": 1,
                "title": 1,
                "score": doc! { "$meta": "vectorSearchScore"
                }
            }
        }
    ];

    let coll = client.database("sample_mflix").collection::<Document>("embedded_movies");
    let mut results = coll.aggregate(pipeline).await?;
    while let Some(result) = results.try_next().await? {
        println!("{}", result);
    }

    Ok(())
}