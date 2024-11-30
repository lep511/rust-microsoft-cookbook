use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 3 {
            return Err("Not enough arguments".into());
        } else {
            let query = args[1].clone();
            let file_path = args[2].clone();
            Ok(Config { query, file_path })
        }
    }
}

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    const MAX_SIZE: u64 = 1024 * 1024 * 10; // 10MB limit
    
    let metadata = fs::metadata(file_path)?;
    if metadata.len() > MAX_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "File too large to read into memory"
        ));
    }
    
    fs::read_to_string(file_path)
}