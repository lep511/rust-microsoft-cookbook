use mongodb::{bson::doc, Client, Collection};
use serde::{ Deserialize, Serialize };
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    cuisine: String,
    address: Address,
    borough: String,
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
    let db_password = env::var("MONGODB_PASS")
        .expect("MONGODB_PASS environment variable not set.");
    
    // Replace the placeholder with your Atlas connection string
    let uri = format!(
        "mongodb+srv://admin:{}@cluster0.y7iwt.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0",
        db_password
    );

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
        name: "Sea Stone Tavern".to_string(),
        cuisine: "Greek".to_string(),
        address: adress,
        borough: "Manhattan".to_string(), // Uncomment this line to see the error
    };

    let res = my_coll.insert_one(doc).await?;
    println!("Inserted a document with _id: {}", res.inserted_id);

    Ok(())
}