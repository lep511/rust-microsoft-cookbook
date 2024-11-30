use std::env;
use minigrep::{ Config, read_file };

fn main() {
    let args: Vec<String> = env::args().collect();
   
    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            return;
        }
    };

    match read_file(&config.file_path) {
        Ok(contents) => contents.lines()
                            .filter(|line| line.contains(&config.query))
                            .for_each(|line| println!("{line}")),
        Err(e) => {
            eprintln!("Error reading file {}. {}", config.file_path, e);
        }
    }
}