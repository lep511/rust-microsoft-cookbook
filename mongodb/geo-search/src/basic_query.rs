use mongodb::{Client, Collection, error::Result};
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;
use std::env;

#[derive(Deserialize, Debug)]
struct Record {
    // Map the JSON "_id" field to this struct field
    _id: ObjectId,
    recrd: String,
    vesslterms: String,
    feature_type: String, // JSON "feature_type" maps directly
    chart: String,
    latdec: f64, // JSON numbers map to Rust numeric types
    londec: f64,
    gp_quality: String,
    depth: f64,
    sounding_type: String,
    history: String,
    quasou: String,
    watlev: String,
    coordinates: Vec<f64>,
}

pub(crate) async fn handler_basic_query() -> Result<()> {
    // Read connection string from environment
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&uri).await?;

    let database = client.database("sample_geospatial");
    let collection: Collection<Record> = database.collection("shipwrecks");

    // Example query to find records with specific criteria
    let filter = doc! { 
        "feature_type": "Wrecks - Submerged, dangerous",
        "chart": "US,US,reprt,L-1-2015",
    };
    let mut cursor = collection.find(filter).await?;
    while let Some(result) = cursor.try_next().await? {
        println!("Feature type: {}, Chart: {}", result.feature_type, result.chart);
        println!("Coordinates: {:?}", result.coordinates);
        println!("----------------------------------------------------");
    }
    // Example query to find records within a certain distance from a point
    let coord_fin = vec![-79.9005833, 9.3723889];
    println!("\n\nCoordinates to find: {:?}\n", coord_fin);
    let point = doc! { "type": "Point", "coordinates": coord_fin };
    let filter = doc! { "coordinates": { "$near": { "$geometry": point, "$maxDistance": 2000 } } };
    let mut cursor = collection.find(filter).await?;
    while let Some(result) = cursor.try_next().await? {
        println!("Object ID: {}", result._id.to_string());
        println!("Feature type: {}, Chart: {}", result.feature_type, result.chart);
        println!("Coordinates: {:?}", result.coordinates);
        println!("----------------------------------------------------");
    }

    Ok(())
}