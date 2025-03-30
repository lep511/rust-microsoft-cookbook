use mongodb::{Client, error::Result};
use mongodb::bson::doc;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    title: String,
    plot: String,
    year: i32,
    genres: Vec<String>,
}

pub(crate) async fn handler_adv_query() -> Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&uri).await?;
    
    // Access the database and collection
    let database = client.database("sample_mflix");
    let collection = database.collection::<Movie>("movies");

    // Create the search pipeline
    // This code example uses the compound operator to combine several operators 
    // into a single query. It has the following search criteria:
    //
    //  - The plot field must contain either Hawaii or Alaska.
    //  - The plot field must contain a four-digit number, such as a year.
    //  - The year field must be greater than or equal to 2000.
    //  - The genres field must not contain either Comedy or Romance.
    //  - The title field must not contain Beach or Snow.

    let pipeline = vec![
        doc! {
            "$search": {
                "compound": {
                    "must": [
                        {
                            "text": {
                                "query": ["Hawaii", "Alaska"],
                                "path": "plot"
                            }
                        },
                        {
                            "regex": {
                                "query": "([0-9]{4})",
                                "path": "plot",
                                "allowAnalyzedField": true
                            }
                        },
                        {
                            "range": {
                                "path": "year",
                                "gte": 2000,
                            }
                        }
                    ],
                    "mustNot": [
                        {
                            "text": {
                                "query": ["Comedy", "Romance"],
                                "path": "genres"
                            }
                        },
                        {
                            "text": {
                                "query": ["Beach", "Snow"],
                                "path": "title"
                            }
                        }
                    ]
                }
            }
        },
        doc! {
            "$project": {
                "_id": 0,
                "title": 1,
                "plot": 1,
                "genres": 1,
                "year": 1,
            }
        }
    ];
    
    // Execute the aggregation pipeline
    let mut results = collection.aggregate(pipeline).await?;

    // Process the results
    while let Some(result) = results.try_next().await? {
        // Convert BSON document to a Movie struct
        match mongodb::bson::from_document::<Movie>(result) {
            Ok(movie) => {
                println!("Title: {} | Plot: {} | Year: {:?} | Genres: {:?}", 
                movie.title, 
                movie.plot, 
                movie.year,
                movie.genres
                );
            },
            Err(e) => {
                println!("Error deserializing document: {}", e);
            }
        }
    }

    Ok(())
}