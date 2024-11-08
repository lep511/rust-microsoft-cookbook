use mongodb::{bson::doc, Client, Collection};
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

pub async fn mongodb_connect(chat_userid: String) -> mongodb::error::Result<MongoResponse> {
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
            let user_data = String::from("Input 1");
            
            // let doc = UserHistory {
            //     user_id: chat_userid,
            //     chat_history: user_data.clone(),
            //     order_state: None,
            // };
            
            // let res = my_coll.insert_one(doc).await?;
            // println!("Inserted a document with _id: {}", res.inserted_id);
            let mongo_response = MongoResponse {
                user_data: user_data,
                chat_count: 1
            };

            Ok(mongo_response)
        }
    }    
}


