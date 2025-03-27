use mongodb::{bson::doc, Client, Collection};
use serde::{ Deserialize, Serialize };
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Grade {
    #[serde(rename = "date")]
    date: Option<DateWrapper>,
    grade: Option<String>,
    score: Option<i32>
}

#[derive(Serialize, Deserialize, Debug)]
struct DateWrapper {
    #[serde(rename = "$date")]
    timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    cuisine: String,
    grades: Vec<Grade>,
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
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
    
    
    let mut restaurants = Vec::new();

    if let Ok(lines) = read_lines("restaurantes.json") {
        for line in lines {
            match line {
                Ok(json_str) => {
                    match serde_json::from_str::<Restaurant>(&json_str) {
                        Ok(restaurant) => restaurants.push(restaurant),
                        Err(e) => eprintln!("Failed to parse JSON: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to read line: {}", e),
            }
        }
    } else {
        eprintln!("Failed to read file.");
    }

    let client = Client::with_uri_str(uri).await?;
    
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");

    let insert_many_result = my_coll.insert_many(&restaurants).await?;   
    println!("Total of documents inserted: {}", insert_many_result.inserted_ids.len());
    
    Ok(())

}