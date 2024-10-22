use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let patterns = vec![
        Regex::new(r"\b3[47][0-9]{13}\b").unwrap(),                                     // American Express
        Regex::new(r"\b501105\d{10}\b").unwrap(),                                       // Argencard
        Regex::new(r"\b589657\d{10}\b").unwrap(),                                       // Cabal 1
        Regex::new(r"\b627170\d{10}\b").unwrap(),                                       // Cabal 2
        Regex::new(r"\b625094\d{10}\b").unwrap(),                                       // China Union Pay
        Regex::new(r"\b5610[0-9]{12}\b").unwrap(),                                      // Discover 1
        Regex::new(r"\b6(?:011|5[0-9]{2})[0-9]{12,13}\b").unwrap(),                     // Discover 2
        Regex::new(r"\b3(?:0[0-5]|[68][0-9])[0-9]{11}\b").unwrap(),                     // Diners Club
        Regex::new(r"\b3(?:2131|1800|[0-9]{4})[0-9]{11}\b").unwrap(),                   // JCB
        Regex::new(r"\b5[1-5][0-9]{14}\b").unwrap(),                                    // Mastercard 5-xxx
        Regex::new(r"\b2[1-5][0-9]{14}\b").unwrap(),                                    // Mastercard 2-xxx     
        Regex::new(r"\b4[0-9]{12}(?:[0-9]{3})?\b").unwrap(),                            // Visa
        Regex::new(r"\b[0-9]{3}\-[0-9]{2}\-[0-9]{4}\b").unwrap(),                       // SSN (XXX-XX-XXXX)
        Regex::new(r"\b[0-9]{3}\ [0-9]{2}\ [0-9]{4}\b").unwrap(),                       // SSN (XXX-XX-XXXX)
        Regex::new(r"\b[0-9]{10}\b").unwrap(),                                          // 10-digit number
        Regex::new(r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b").unwrap(),     // Email address
        Regex::new(r"\+1 \(\d{3}\) \d{3}-\d{4}").unwrap(),                                      // +1 (222) 222-2222
        Regex::new(r"\d{3} ?– ?\d{3}-\d{4}").unwrap(),                                          // 203 – 688-5500
        Regex::new(r"\d{3} \d \d{3} \d{3} \d{4}").unwrap(),                                     // 010 1 718 222 2222
    ];


    let path = Path::new("data.csv");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut output = File::create("output.csv")?;
    let mut total_changes = 0;

    for line in reader.lines() {
        let mut line = line?;
        for re in &patterns {
            let matches = re.find_iter(&line).count();
            if matches > 0 {
                total_changes += matches;
                line = re.replace_all(&line, "######").to_string();
            }
        }
        writeln!(output, "{}", line)?;
    }

    println!("Processed CSV file saved as output.csv");
    println!("Total changes made: {}", total_changes);
    Ok(())
}