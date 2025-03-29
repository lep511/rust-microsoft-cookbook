// src/main.rs

use serde::Deserialize;
use fancy_regex::Regex;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PiiFilterError {
    #[error("Failed to read pattern file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Failed to parse JSON pattern file: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Failed to compile regex for pattern '{name}': {source}")]
    RegexCompilationError { name: String, source: fancy_regex::Error },

    #[error("Pattern '{0}' has no expressions defined.")]
    EmptyExpressionsError(String),
}

#[derive(Deserialize, Debug, Clone)]
struct PiiPatternDefinition {
    name: String,
    expressions: Vec<String>,
}

#[derive(Debug)]
struct CompiledPiiPattern {
    name: String,
    regex: Regex,
}

fn load_pattern_definitions<P: AsRef<Path>>(file_path: P) -> Result<Vec<PiiPatternDefinition>, PiiFilterError> {
    let json_content = fs::read_to_string(file_path)?;
    let patterns: Vec<PiiPatternDefinition> = serde_json::from_str(&json_content)?;
    Ok(patterns)
}

fn compile_pii_patterns(definitions: &[PiiPatternDefinition]) -> Result<Vec<CompiledPiiPattern>, PiiFilterError> {
    let mut compiled_patterns = Vec::new();

    for definition in definitions {
        if definition.expressions.is_empty() {
            return Err(PiiFilterError::EmptyExpressionsError(definition.name.clone()));
        }

        let combined_regex_str = definition.expressions.join("|");
        match Regex::new(&combined_regex_str) {
            Ok(re) => {
                compiled_patterns.push(CompiledPiiPattern {
                    name: definition.name.clone(),
                    regex: re,
                });
            }
            Err(e) => {
                return Err(PiiFilterError::RegexCompilationError {
                    name: definition.name.clone(),
                    source: e,
                });
            }
        }
    }

    Ok(compiled_patterns)
}

fn filter_pii(text: &str, compiled_patterns: &[CompiledPiiPattern], replacement: &str) -> String {
    // Start with a mutable copy of the input text.
    let mut current_text = text.to_string();

    // Iterate through each compiled PII pattern
    for pattern in compiled_patterns {
        // Replace PII with the replacement string and update current_text
        current_text = pattern.regex.replace_all(&current_text, replacement).into_owned();
    }

    current_text
}

fn main() -> Result<(), PiiFilterError> {
    let pattern_file_path = "pii_patterns.json";
    let replacement_text = "[REDACTED]";

    let input_text = r#"
    John Doe's SSN is 123-45-6789 and his phone number is (555) 123-4567.
    His email is john.doe.123@example.com.
    He lives at 123 Main St, Anytown.
    His credit card is 4444-5555-6666-7777.
    Another contact: jane@company.org, phone +1 987 654 3210.
    Invalid SSN: 000-00-0000. Invalid Card: 1111222233334444.
    Address example: 456 South West Oak Ave
    American Express Card: 370011223344556
    Hello, my name is David Johnson and I live in Maine
    My credit card number is 4095-2609-9393-4932 and my crypto wallet id is 16Yeky6GMjeNkAiNcBY7ZhrLoMSgg1BoyZ
    On September 18 I visited microsoft.com and sent an email to test@presidio.site,  from the IP 192.168.0.1
    My passport: 191280342 and my phone number: (212) 555-1234
    This is a valid International Bank Account Number: IL150120690000003111111
    Can you please check the status on bank account 954567876544?
    Kate's social security number is 078-05-1126.  Her driver license? it is 1234567A
    Hi, My name is John.
    "#;

    println!("--- Original Text ---");
    println!("{}", input_text);

    let definitions = load_pattern_definitions(pattern_file_path)?;
    println!("Successfully loaded {} PII pattern definitions.", definitions.len());

    let compiled_patterns = compile_pii_patterns(&definitions)?;
    println!("Successfully compiled {} PII patterns.", compiled_patterns.len());

    println!("\n--- Filtering Text (Replacing with '{}') ---", replacement_text);
    let filtered_output = filter_pii(input_text, &compiled_patterns, replacement_text);

    println!("\n--- Filtered Text ---");
    println!("{}", filtered_output);

    Ok(())
}