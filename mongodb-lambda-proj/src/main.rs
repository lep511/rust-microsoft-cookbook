use mongodb::{bson::doc, Client, Collection};
use serde::{ Deserialize, Serialize };
use std::env;
use std::fs;

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

fn read_file_to_string(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("Failed to read file")
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

    let file_contents = read_file_to_string("sample_data.txt");

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
        comment: Some(file_contents), // Uncomment this line to see the error
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