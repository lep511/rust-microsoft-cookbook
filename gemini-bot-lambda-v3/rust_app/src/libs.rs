use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MedicalData {
    pub user_id: String,
    pub medical_info: String,
    pub medical_terms: Vec<MedicalTerms>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MedicalTerms {
    pub code_type: String,
    pub code_value: String,
    pub code_explain: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MedicalDummie {
    pub medical_terms: Vec<MedicalTerms>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileData {
    pub user_id: String,
    pub medical_info: String
}