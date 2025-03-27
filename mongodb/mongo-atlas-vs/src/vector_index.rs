use std::ops::Index;
use std::time::Duration;
use futures::{TryStreamExt};
use mongodb::{bson::{Document, doc}, Client, Collection, SearchIndexModel};
use mongodb::SearchIndexType::VectorSearch;
use tokio::time::sleep;
use std::env;

pub(crate) async fn create_vector_index() -> mongodb::error::Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // Create a new client and connect to the server
    let client = Client::with_uri_str(&uri).await?;

    // Get a handle on the movies collection
    let database = client.database("sample_mflix");
    let collection: Collection<Document> = database.collection("embedded_movies");

    let index_name = "vector_index";
    let mut cursor = collection.list_search_indexes().await?;

    while let Some(index) = cursor.try_next().await? {
        if let Some(index_type) = index.get_str("type").ok() {
            if index_type == "vectorSearch" {
                if let Some(name) = index.get_str("name").ok() {
                    if name == index_name {
                        println!("Atlas Vector Search index '{}' exist.\n", index_name);
                        return Ok(());
                    }
                }
            }
        }
    }

    let search_index_def = SearchIndexModel::builder()
        .definition(doc! {
            "fields": vec! {doc! {
                "type": "vector",
                "path": "plot_embedding",
                "numDimensions": 1536,
                "similarity": "dotProduct",
                "quantization": "scalar"
            }}
        })
        .name(index_name.to_string())
        .index_type(VectorSearch)
        .build();

    let models = vec![search_index_def];
    let result = collection.create_search_indexes(models).await;
    if let Err(e) = result {
        eprintln!("There was an error creating the search index: {}", e);
        std::process::exit(1)
    } else {
        println!("New search index named {} is building.", result.unwrap().index(0));
    }

    // Polling for the index to become queryable
    println!("Polling to check if the index is ready. This may take up to a minute...");
    let mut is_index_queryable = false;
    while !is_index_queryable {
        // List the search indexes
        let mut search_indexes = collection.list_search_indexes().await.unwrap();
        // Check if the index is present and queryable
        while let Some(index) = search_indexes.try_next().await.unwrap() {
            let retrieved_name = index.get_str("name");
            if retrieved_name.unwrap().to_string() == index_name {
                is_index_queryable = index.get_bool("queryable").unwrap();
            }
        }
        if !is_index_queryable {
            sleep(Duration::from_secs(5)).await; // Wait for 5 seconds before polling again
        }
    }
    println!("{} is ready for querying.", index_name);

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashMap;
//     use lambda_http::{Request, RequestExt};

//     #[tokio::test]
//     async fn test_generic_http_handler() {
//         let request = Request::default();

//         let response = function_handler(request).await.unwrap();
//         assert_eq!(response.status(), 200);

//         let body_bytes = response.body().to_vec();
//         let body_string = String::from_utf8(body_bytes).unwrap();

//         assert_eq!(
//             body_string,
//             "Hello world, this is an AWS Lambda HTTP request"
//         );
//     }

//     #[tokio::test]
//     async fn test_http_handler_with_query_string() {
//         let mut query_string_parameters: HashMap<String, String> = HashMap::new();
//         query_string_parameters.insert("name".into(), "dataser".into());

//         let request = Request::default()
//             .with_query_string_parameters(query_string_parameters);

//         let response = function_handler(request).await.unwrap();
//         assert_eq!(response.status(), 200);

//         let body_bytes = response.body().to_vec();
//         let body_string = String::from_utf8(body_bytes).unwrap();

//         assert_eq!(
//             body_string,
//             "Hello dataser, this is an AWS Lambda HTTP request"
//         );
//     }
// }
