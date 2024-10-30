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

    let filter: Document = doc! {
        "address.street": "Sullivan Street",
        "borough": "Manhattan"
    };

    let update = doc! { "$set": doc! { "near_me": true } };
    let res = my_coll.update_many(filter, update).await?;
    println!("Updated documents: {}", res.modified_count);

    Ok(())
}
