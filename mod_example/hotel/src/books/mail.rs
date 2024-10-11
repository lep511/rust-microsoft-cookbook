#[derive(Debug)]
pub struct Mail {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl Mail {
    pub fn new(from: String, to: String, subject: String, body: String) -> Mail {
        Mail {
            from,
            to,
            subject,
            body,
        }
    }
}