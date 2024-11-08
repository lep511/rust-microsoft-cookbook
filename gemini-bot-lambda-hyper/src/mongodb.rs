use mongodb::{
    bson::{ doc, Document }, results::UpdateResult, Client, Collection
};
use serde::{ Deserialize, Serialize };
use crate::gemini::OrderState;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserHistory {
    pub user_id: String,
    pub chat_history: String,
    pub chat_count: i32,
    pub order_state: Option<OrderState>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MongoResponse {
    pub user_data: String,
    pub chat_count: i32
}

pub async fn mongodb_connect(chat_userid: &str) -> mongodb::error::Result<MongoResponse> {
    let db_password = env::var("MONGODB_PASS")
        .expect("MONGODB_PASS environment variable not set.");

    let user_name = env::var("USER_NAME")
        .expect("USER_NAME environment variable not set.");
    
    let uri = format!(
        "mongodb+srv://{}:{}@cluster0.klhsa.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0",
        user_name,
        db_password
    );

    let client = Client::with_uri_str(uri).await?;

    let my_coll: Collection<UserHistory> = client
        .database("llm")
        .collection("coffe_shops");

    let result = my_coll.find_one(
        doc! { "user_id": &chat_userid }
    ).await?;

    match result {
        Some(doc) => {
            println!("Found a document with _id: {}", doc.user_id);
            let mongo_response = MongoResponse {
                user_data: doc.chat_history,
                chat_count: doc.chat_count
            };
            Ok(mongo_response)
        },
        None => {
            let user_data = String::from("## Real Conversation\n");

            let doc = UserHistory {
                user_id: chat_userid.to_string(),
                chat_history: user_data.clone(),
                chat_count: 1,
                order_state: None,
            };
            
            let resp = my_coll.insert_one(doc).await?;
            println!("Inserted a document with _id: {}", resp.inserted_id);
            
            let mongo_response = MongoResponse {
                user_data: user_data,
                chat_count: 1
            };

            Ok(mongo_response)
        }
    }    
}

pub async fn mongodb_update(chat_userid: &str, update_chat: &str, nc_count: i32) -> mongodb::error::Result<MongoResponse> {
    let db_password = env::var("MONGODB_PASS")
        .expect("MONGODB_PASS environment variable not set.");

    let user_name = env::var("USER_NAME")
        .expect("USER_NAME environment variable not set.");

    let uri = format!(
        "mongodb+srv://{}:{}@cluster0.klhsa.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0",
        user_name,
        db_password
    );

    let client = Client::with_uri_str(uri).await?;

    let my_coll: Collection<UserHistory> = client
        .database("llm")
        .collection("coffe_shops");

    let filter: Document = doc! { "user_id": chat_userid };
    let update: Document = doc! { 
        "$set": doc! {
            "chat_history": update_chat,
            "chat_count": nc_count + 1,
        } 
    };
    
    let res: UpdateResult = my_coll.update_one(filter, update).await?;
    println!("Updated documents: {}", res.modified_count);

    let mongo_response = MongoResponse {
        user_data: "update".to_string(),
        chat_count: nc_count + 1,
    };

    Ok(mongo_response)
}