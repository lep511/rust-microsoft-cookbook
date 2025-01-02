use mongodb::{bson::{doc, Document}, Collection, Client};
// use serde::{ Deserialize, Serialize };
use futures::TryStreamExt;
use gemini::EmbedGemini;
use std::env;

mod llmerror;
mod gemini;

async fn get_embedding(text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let llm = EmbedGemini::new("text-embedding-004")?;
    let response = llm.embed_content(text).await?;
    // Wait 3 sec
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    match response.embedding {
        Some(embedding) => Ok(embedding.values),
        None => Err("Failed to get embedding".into()),
    }
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Connect to MongoDB
    let uri = env::var("MONGODB_SRV")
        .expect("MONGODB_SRV environment variable not set.");

    let client = Client::with_uri_str(uri).await?;

    let movies: Collection<Document> = client
        .database("sample_mflix")
        .collection("test_movies");

    // let data = vec![
    //     "Titanic: The story of the 1912 sinking of the largest luxury liner ever built",
    //     "The Lion King: Lion cub and future king Simba searches for his identity",
    //     "Avatar: A marine is dispatched to the moon Pandora on a unique mission",
    //     "The Matrix: A computer hacker learns from mysterious rebels about the true nature of his reality and his role in the war against its controllers",
    //     "Back to the Future: A teenager is accidentally sent 30 years into the past in a time-traveling DeLorean",
    //     "Gladiator: A former Roman General sets out to exact vengeance against the corrupt emperor who murdered his family",
    //     "The Dark Knight: Batman faces a new enemy, The Joker, who is an anarchist mastermind",
    //     "The Godfather: The aging patriarch of an organized crime dynasty transfers control of his clandestine empire to his reluctant son",
    //     "Batman Begins: After training with his mentor, Batman begins his fight to free crime-ridden Gotham City from corruption",
    //     "Taxi Driver: A mentally unstable veteran works as a nighttime taxi driver in New York City",
    // ];

    // let mut inserted_doc_count = 0;
    // for text in data {
    //     let embedding = match get_embedding(text).await {
    //         Ok(embedding) => embedding,
    //         Err(e) => {
    //             println!("Error: {}", e);
    //             continue;
    //         }
    //     };

    //     let doc = doc! {
    //         "plot": text,
    //         "plot_embedding": embedding
    //     };
    //     match movies.insert_one(doc).await {
    //         Ok(_) => inserted_doc_count += 1,
    //         Err(e) => {
    //             println!("Error: {}", e);
    //             continue;
    //         }
    //     }
    // }

    // println!("Inserted {} documents.", inserted_doc_count);

    let prompt = "Nueva York";
    let query_vector = match get_embedding(prompt).await {
        Ok(embedding) => embedding,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };
    
    let pipeline  = vec! [
        doc! {
            "$vectorSearch": doc! {
            "queryVector": query_vector,
            "path": "plot_embedding",
            "numCandidates": 150,
            "index": "vector_index_test",
            "limit": 3
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

    let mut results = movies.aggregate(pipeline).await?;

    while let Some(result) = results.try_next().await? {
        println!("{}", result);
    }

    Ok(())
}