use mongodb::{
    bson::{ doc, Document }, results::UpdateResult, Client, Collection
};
use std::env;

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
    
    let my_coll: Collection<Document> = client
        .database("sample_restaurants")
        .collection("restaurants");    

    // The following code adds the price field to a document in which the value of the name field 
    // is "O Lavrador Restaurant". MongoDB updates the first document that matches the query filter.   
    
    let filter: Document = doc! { "name": "O Lavrador Restaurant" };
    let update: Document = doc! { "$set": doc! {"price": "$$$"} };
    let res: UpdateResult = my_coll.update_one(filter, update).await?;
    println!("Updated documents: {}", res.modified_count);
    
    Ok(())
}
