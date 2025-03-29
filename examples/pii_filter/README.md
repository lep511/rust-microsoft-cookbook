# Rust PII Filter Example

A command-line tool written in Rust demonstrating how to load PII (Personally Identifiable Information) detection patterns from a JSON configuration file, compile them into regular expressions, and use them to filter (redact) sensitive data from input text.

Based in **[langkit](https://github.com/whylabs/langkit/tree/main)**.

## Functionality

This tool performs the following steps:

1.  **Loads PII Patterns:** Reads a list of PII types and their corresponding regular expression patterns from a JSON file (`pii_patterns.json` by default).
2.  **Compiles Regex:** Compiles the regular expression strings from the JSON file into efficient `Regex` objects using the Rust `regex` crate. If a PII type has multiple expression strings, they are combined using the `|` (OR) operator into a single `Regex` object for that type.
3.  **Filters Text:** Takes an input text string and iterates through the compiled PII patterns. For each pattern, it finds all matches in the text and replaces them with a predefined placeholder string (e.g., `[REDACTED]`).
4.  **Outputs Result:** Prints the original text and the final text after all filtering patterns have been applied.

## Features

*   **JSON Configuration:** Easily define and modify PII patterns without changing the Rust code.
*   **Multiple Expressions per Type:** Supports defining several regex variations for a single PII type (e.g., different phone number formats).
*   **Customizable Replacement:** The text used to replace detected PII can be easily changed in the code.
*   **Error Handling:** Includes basic error handling for file reading, JSON parsing, and regex compilation issues.

## Prerequisites

*   **Rust:** Ensure you have Rust and Cargo installed. You can get them from [https://rustup.rs/](https://rustup.rs/).

## Installation & Setup

1.  **Clone the Repository (or create the project):**
    ```bash
    # If you have the code from the example:
    cd pii_filter_example
    # Or, if cloning from a Git repo:
    # git clone <your-repo-url>
    # cd <repo-name>
    ```

2.  **Build the Project:**
    ```bash
    cargo build
    ```
    (Running the project with `cargo run` will also build it if necessary).

## Configuration (`pii_patterns.json`)

The PII detection patterns are defined in the `pii_patterns.json` file located in the project root. The file should contain a JSON array of objects, where each object represents a PII type:

```json
[
  {
    "name": "Descriptive Name (e.g., SSN)",
    "expressions": [
      "regex_pattern_string_1",
      "regex_pattern_string_2"
      // Add more regex variations for this type if needed
    ]
  },
  {
    "name": "Another PII Type (e.g., Email)",
    "expressions": [
      "email_regex_pattern"
    ]
  }
  // ... more pattern objects
]