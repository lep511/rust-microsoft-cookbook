#[derive(Debug)]
pub struct Phone {
    pub phone_number: String,
    pub phone_type: String
}

impl Phone {
    pub fn new(phone_number: String, phone_type: String) -> Phone {
        Phone {
            phone_number,
            phone_type
        }
    }
}