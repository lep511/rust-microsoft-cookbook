use minigrep::*;
use std::fs::File;
use std::io::Write;

#[test]
fn test_config_new_valid() {
    let args = vec![
        String::from("program"),
        String::from("query"),
        String::from("file.txt"),
    ];
    let config = Config::new(&args).unwrap();
    assert_eq!(config.query, "query");
    assert_eq!(config.file_path, "file.txt");
}

#[test]
fn test_config_new_insufficient_args() {
    let args = vec![String::from("program"), String::from("query")];
    let result = Config::new(&args);
    assert!(result.is_err());
}

#[test]
fn test_read_file_success() {
    // Create a temporary test file
    let test_content = "Hello, World!";
    let test_file = "test_file.txt";
    
    let mut file = File::create(test_file).unwrap();
    file.write_all(test_content.as_bytes()).unwrap();

    // Test reading the file
    let result = read_file(test_file);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), test_content);

    // Clean up
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_read_file_nonexistent() {
    let result = read_file("nonexistent_file.txt");
    assert!(result.is_err());
}

#[test]
fn test_read_file_too_large() {
    // Create a temporary file that exceeds the size limit
    let test_file = "large_test_file.txt";
    let file = File::create(test_file).unwrap();
    
    // Set the file size to be larger than MAX_SIZE
    file.set_len(1024 * 1024 * 11).unwrap(); // 11MB

    let result = read_file(test_file);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.to_string(), "File too large to read into memory");
    }

    // Clean up
    std::fs::remove_file(test_file).unwrap();
}