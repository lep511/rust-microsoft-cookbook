use mongodb::{bson::doc, Client, Collection};
use serde::{ Deserialize, Serialize };
use crate::gemini::OrderState;
use crate::bot::guideline_bot;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserHistory {
    pub user_id: String,
    pub content: String,
    pub order_state: Option<OrderState>,
}

pub async fn mongodb_connect(chat_userid: String) -> mongodb::error::Result<String> {
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

    let user_data = match guideline_bot() {
        Ok(data) => data,
        Err(e) => panic!("Error in load user data: {e}")
    };

    let doc = UserHistory {
        user_id: chat_userid,
        content: user_data.clone(),
        order_state: None,
    };

    let res = my_coll.insert_one(doc).await?;
    println!("Inserted a document with _id: {}", res.inserted_id);

    Ok(user_data)
}


