use csv::ReaderBuilder;
use std::fs::File;
use csv::Reader;
use std::error::Error;

pub fn read_csv_file(
    csv_file_path: &str,
    delimiter: u8,
    has_headers: bool,
) -> Result<Reader<File>, Box<dyn std::error::Error>> {
    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(has_headers) 
        .from_path(csv_file_path)?;
    Ok(reader)
}

fn main()-> Result<(), Box<dyn Error>> {
    let csv_file_path = "dataset.csv";
    let mut rdr = match read_csv_file(csv_file_path, b',', true) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading CSV file: {}", e);
            return Ok(());
        }
    };

    let mut query = String::new();

    // Track if we've processed at least one row
    let mut first_row = true;

    let mut n = 0;
    for result in rdr.records() {
        let record = result?;

        // If this isn't the first row, add a UNION ALL
        if !first_row {
            query.push_str("\nUNION ALL\n");
        } else {
            first_row = false;
        }

        query.push_str("SELECT ");

        for (i, field) in record.iter().enumerate() {
            // Add a comma if this isn't the first field
            if i > 0 {
                query.push_str(", ");
            }

            // Format the field based on its content
            if field.is_empty() {
                // Empty fields become NULL
                query.push_str("NULL");
            } else if field == "NULL" {
                // Explicit NULL values
                query.push_str("NULL");
            } else if field.parse::<i64>().is_ok() {
                // Integer values (no quotes)
                query.push_str(field);
            } else if field.parse::<f64>().is_ok() {
                // Float values (no quotes)
                query.push_str(field);
            } else if field.to_lowercase() == "true" || field.to_lowercase() == "false" {
                // Boolean values (no quotes, lowercase)
                query.push_str(&field.to_lowercase());
            } else if field.to_lowercase() == "yes" {
                // "yes" becomes true
                query.push_str("true");
            } else if field.to_lowercase() == "no" {
                // "no" becomes false
                query.push_str("false");
            } else if field.starts_with("TIMESTAMP ") {
                // Already formatted timestamp values
                query.push_str(field);
            } else if field.contains("-") && field.contains(":") && field.len() >= 16 {
                // Likely a timestamp that needs formatting
                let field_fmt = format!("TIMESTAMP '{}'", field);
                query.push_str(&field_fmt);
            } else {
                // String values (quoted)
                let field_fmt = field.replace("'", "''");
                let field_fmt = format!("'{}'", field_fmt);
                query.push_str(&field_fmt);
            }
        }
        if n == 5 {
            break;
        }
        n += 1;
    }

    println!("{}", query);
    Ok(())
}
