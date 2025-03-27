use mongodb::{ 
    bson::{Document, doc},
    Client
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

    let file_path = "vector_data.json";
    let vector_data = handle_file(file_path).await;
    let parse_vector = vector_data.vector;
    
    let pipeline  = vec! [
        doc! {
            "$vectorSearch": doc! {
            "queryVector": parse_vector,
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

async fn read_json_vector<P: AsRef<Path>>(
    path: P
) -> Result<VectorData, Box<dyn Error>> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let data: VectorData = serde_json::from_str(&contents)?;

    Ok(data)
}

async fn handle_file(file_path: &str) -> VectorData {  
    match read_json_vector(file_path).await {
        Ok(data) => {
            println!("JSON file read successfully: {}", file_path);
            data
        }
        Err(e) => {
            println!("Error reading or parsing JSON file: {}", e);
            VectorData { vector: vec![] }
        }
    }
}