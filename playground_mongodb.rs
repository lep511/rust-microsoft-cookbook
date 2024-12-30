use mongodb::{bson::doc, sync::Client};
use chrono::Utc;

fn main() -> mongodb::error::Result<()> {
    // Connect to the MongoDB instance.
    let client = Client::with_uri_str("mongodb://localhost:27017/")?;
    let db = client.database("mongodbVSCodePlaygroundDB");
    let collection = db.collection("sales");

    // Insert a few documents into the sales collection.
    collection.insert_many(vec![
        doc! { "item": "abc", "price": 10, "quantity": 2, "date": Utc.ymd(2014, 3, 1).and_hms(8, 0, 0) },
        doc! { "item": "jkl", "price": 20, "quantity": 1, "date": Utc.ymd(2014, 3, 1).and_hms(9, 0, 0) },
        doc! { "item": "xyz", "price": 5, "quantity": 10, "date": Utc.ymd(2014, 3, 15).and_hms(9, 0, 0) },
        doc! { "item": "xyz", "price": 5, "quantity": 20, "date": Utc.ymd(2014, 4, 4).and_hms(11, 21, 39) },
        doc! { "item": "abc", "price": 10, "quantity": 10, "date": Utc.ymd(2014, 4, 4).and_hms(21, 23, 13) },
        doc! { "item": "def", "price": 7.5, "quantity": 5, "date": Utc.ymd(2015, 6, 4).and_hms(5, 8, 13) },
        doc! { "item": "def", "price": 7.5, "quantity": 10, "date": Utc.ymd(2015, 9, 10).and_hms(8, 43, 0) },
        doc! { "item": "abc", "price": 10, "quantity": 5, "date": Utc.ymd(2016, 2, 6).and_hms(20, 20, 13) },
    ], None)?;

    // Run a find command to view items sold on April 4th, 2014.
    let sales_on_april_4th = collection.count_documents(
        doc! { "date": { "$gte": Utc.ymd(2014, 4, 4).and_hms(0, 0, 0), "$lt": Utc.ymd(2014, 4, 5).and_hms(0, 0, 0) } },
        None,
    )?;

    // Print a message to the output window.
    println!("{} sales occurred in 2014.", sales_on_april_4th);

    // Run an aggregation and open a cursor to the results.
    let pipeline = vec![
        doc! { "$match": { "date": { "$gte": Utc.ymd(2014, 1, 1).and_hms(0, 0, 0), "$lt": Utc.ymd(2015, 1, 1).and_hms(0, 0, 0) } } },
        doc! { "$group": { "_id": "$item", "totalSaleAmount": { "$sum": { "$multiply": [ "$price", "$quantity" ] } } } },
    ];
    let cursor = collection.aggregate(pipeline, None)?;

    for result in cursor {
        println!("{:?}", result?);
    }

    Ok(())
}