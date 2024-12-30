use mongodb::{bson::{doc, DateTime}, Client, Collection};
use serde::{ Deserialize, Serialize };
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Sale {
    item: String,
    price: f64,
    quantity: i32,
    date: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
struct SaleGroup {
    _id: String,
    total_sale_amount: f64,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Connect to MongoDB
    let uri = env::var("MONGODB_SRV")
        .expect("MONGODB_SRV environment variable not set.");

    let client = Client::with_uri_str(uri).await?;

    let sales: Collection<Sale> = client
        .database("sample_sales")
        .collection("sales");

    // Insert documents
    let documents = vec![
        Sale {
            item: "abc".to_string(),
            price: 10.0,
            quantity: 2,
            date: DateTime::from_millis(1393664400000), // 2014-03-01T08:00:00Z
        },
        Sale {
            item: "jkl".to_string(),
            price: 20.0,
            quantity: 1,
            date: DateTime::from_millis(1393668000000), // 2014-03-01T09:00:00Z
        },
        Sale {
            item: "xyz".to_string(),
            price: 5.0,
            quantity: 10,
            date: DateTime::from_millis(1394874000000), // 2014-03-15T09:00:00Z
        },
        Sale {
            item: "xyz".to_string(),
            price: 5.0,
            quantity: 20,
            date: DateTime::from_millis(1396611699736), // 2014-04-04T11:21:39.736Z
        },
        Sale {
            item: "abc".to_string(),
            price: 10.0,
            quantity: 10,
            date: DateTime::from_millis(1396647793331), // 2014-04-04T21:23:13.331Z
        },
        Sale {
            item: "def".to_string(),
            price: 7.5,
            quantity: 5,
            date: DateTime::from_millis(1433397493000), // 2015-06-04T05:08:13Z
        },
        Sale {
            item: "def".to_string(),
            price: 7.5,
            quantity: 10,
            date: DateTime::from_millis(1441874580000), // 2015-09-10T08:43:00Z
        },
        Sale {
            item: "abc".to_string(),
            price: 10.0,
            quantity: 5,
            date: DateTime::from_millis(1454790013000), // 2016-02-06T20:20:13Z
        },
    ];

    let insert_many_result = sales.insert_many(&documents).await?;   
    println!("Total of documents inserted: {}", insert_many_result.inserted_ids.len());
    
    Ok(())
}