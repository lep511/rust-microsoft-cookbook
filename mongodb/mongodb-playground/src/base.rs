use mongodb::{bson::doc, Client, Collection};
use serde::{ Deserialize, Serialize };
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    cuisine: String,
    address: Address,
    borough: String,
    comment: Option<String>, // Make comment optional
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    building: String,
    coord: (f64, f64),
    street: String,
    zipcode: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Connect to MongoDB
    let uri = env::var("MONGODB_SRV")
        .expect("MONGODB_SRV environment variable not set.");

    let client = Client::with_uri_str(uri).await?;

    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let adress = Address {
        building: "2062".to_string(),
        coord: (-73.9817368, 40.77763660000001),
        street: "Broadway".to_string(),
        zipcode: "10023".to_string(),
    };
    
    let doc = Restaurant {
        name: "Sea Stone New Age".to_string(),
        cuisine: "Chicken-kiken".to_string(),
        address: adress,
        borough: "Manhattan".to_string(),
        comment: Some("This is a comment".to_string()),
    };

    let res = my_coll.insert_one(doc).await?;
    println!("Inserted a document with _id: {}", res.inserted_id);

    // Get document by id
    let filter = doc! { "_id": res.inserted_id };
    let doc = my_coll.find_one(filter).await?;
    if let Some(doc) = doc {
        println!("Found a document: {:?}", doc.comment);
    } else {
        println!("Document not found");
    }
    
    Ok(())
}