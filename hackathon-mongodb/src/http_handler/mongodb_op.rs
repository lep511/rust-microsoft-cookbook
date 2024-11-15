use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

pub async fn manage_mongodb(uri: String, action: &str, db_name: &str, collection_name: &str) -> mongodb::error::Result<()> {

    match action {
        "insert" => {
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
                .database(db_name)
                .collection(collection_name);
        
            let insert_many_result = my_coll.insert_many(&restaurants).await?;   
            println!("Total of documents inserted: {}", insert_many_result.inserted_ids.len());

            Ok(())
        }
        "find" => {
            let client = Client::with_uri_str(uri).await?;

            let my_coll: Collection<Restaurant> = client
                .database(db_name)
                .collection(collection_name);

            let result = my_coll.find_one(
                doc! { "name": "Wendy'S" }
            ).await?;
            
            println!("{:#?}", result);

            Ok(())
        }
        _ => Ok(()),
    }
    
}