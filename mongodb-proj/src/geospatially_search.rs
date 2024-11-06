use mongodb::{ bson::doc, Client, Collection, IndexModel };
use serde::{ Deserialize, Serialize };
use std::env;
use futures::TryStreamExt;

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

    let index = IndexModel::builder()
        .keys(doc! { "address.coord": "2dsphere" })
        .build();
    
    let idx = my_coll.create_index(index).await?;
    
    println!("Created index:\n{}", idx.index_name);

    let location = vec! [-73.98676014809762, 40.76194204809099];

    let query = doc! {"address.coord": 
        doc! { "$near": {
            "$geometry": {
                "type": "Point", "coordinates": location,
                },
            "$maxDistance": 50,
            }
        }
    };
    
    let mut cursor = my_coll.find(query).await?;

    while let Some(doc) = cursor.try_next().await? {
        println!("{:?}", doc);
    }
    
    Ok(())
}