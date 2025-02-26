use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Bundle {
    entry: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    #[serde(rename = "fullUrl")]
    full_url: String,
    resource: Patient,
}

#[derive(Debug, Serialize, Deserialize)]
struct Patient {
    name: Vec<Name>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Name {
    family: Option<String>,         // Surname
    given: Option<Vec<String>>,     // Name
}
