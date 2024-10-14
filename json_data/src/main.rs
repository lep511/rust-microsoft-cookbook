use serde::{Deserialize, Serialize};
use serde_json;

// Nested structs for hierarchical data
#[derive(Deserialize, Serialize, Debug)]
struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Person {
    name: String,
    age: u32,
    #[serde(rename = "mail")]
    email: Option<String>,
    address: Address,
}

// Nested enums for hierarchical data
#[derive(Deserialize, Serialize, Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Music { title: String, artist: String },
}

#[derive(Deserialize, Serialize, Debug)]
struct Library {
    name: String,
    media: Vec<Media>,
}

fn main() {
    // Deserialize JSON data with nested structs
    let json_data = r#"
    {
        "name": "John Doe",
        "age": 35,
        "mail": "john.doe@example.com",
        "address": {
            "street": "123 Main St",
            "city": "Anytown",
            "state": "CA",
            "zip": "12345"
        }
    }"#;

    match serde_json::from_str::<Person>(json_data) {
        Ok(person) => {
            println!("Name: {}", person.name);
            println!("Age: {}", person.age);
            println!("Email: {:?}", person.email);
            println!("Address: {:#?}", person.address);
        }
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
        }
    }

    let person: Person = serde_json::from_str(json_data).unwrap();
    
    println!("\n{} have {} years old.\n\n", person.name, person.age);

    // Deserialize JSON data with nested enums
    let json_data = r#"
    {
        "name": "My Personal Library",
        "media": [
            {
                "Book": {
                    "title": "The Great Gatsby",
                    "author": "F. Scott Fitzgerald"
                }
            },
            {
                "Movie": {
                    "title": "The Shawshank Redemption",
                    "director": "Frank Darabont"
                }
            },
            {
                "Music": {
                    "title": "Bohemian Rhapsody",
                    "artist": "Queen"
                }
            }
        ]
    }"#;

    let library: Library = serde_json::from_str(json_data).unwrap();
    println!("{:#?}", library);
}