use mongodb::{ 
    bson::{ Document, doc }, 
    Client, 
    Collection 
};
use std::env;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let db_password = env::var("MONGODB_PASS").expect("MONGODB_PASS environment variable not set.");

    // Replace the placeholder with your Atlas connection string
    let uri = format!(
        "mongodb+srv://admin:{}@cluster0.y7iwt.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0",
        db_password
    );

    let client = Client::with_uri_str(uri).await?;

    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let filter =
        doc! { "$and": [
           doc! { "name": "Haagen-Dazs" },
           doc! { "borough": "Brooklyn" }
       ]
    };
    let result = my_coll.delete_one(filter).await?;
    println!("Deleted documents: {}", result.deleted_count);
    
    Ok(())
    
}
