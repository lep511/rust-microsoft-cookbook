use regex::Regex;
use reqwest::{Client, Response};
use serde_json::json;

fn replace_raw_escapes(sentence: &str) -> String {  
    const ESCAPE_CHARS: [char; 18] = [
        '\\', '_', '*', '[', ']', '(', ')', '~', '>', '#', 
        '+', '-', '=', '|', '{', '}', '.', '!'
    ];
    
    // Preallocate string with estimated capacity
    let estimated_size = sentence.len() * 2;
    let mut new_sentence = String::with_capacity(estimated_size);
    
    // Escape special characters
    sentence.chars().for_each(|c| {
        if ESCAPE_CHARS.contains(&c) {
            new_sentence.push('\\');
            new_sentence.push(c);
        } else {
            new_sentence.push(c);
        }
    });
    
    // Convert to UTF-16
    let utf16_string: Vec<u16> = new_sentence.encode_utf16().collect();
    
    // If you need to convert back to String:
    let decoded_string = String::from_utf16(&utf16_string).unwrap_or_default();
    let decoded_string = decoded_string.replace("\\*\\*", "*");
    decoded_string
}

pub async fn send_telegram_message(
    text: &str, 
) -> Result<(), Box<dyn std::error::Error>> {
    let telegram_client = Client::new();
    let bot_token = "7796241975:AAEnE3G8IaUhx-HydXlp5Yc0Fr8OQ0nHE3k";
    let telegram_api_url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let chat_id = "795876358";

    let text = replace_raw_escapes(&text);
    
    let mut message_body = json!({
        "chat_id": chat_id,
        "text": text,
        "parse_mode": "markdownV2"
    });

    let response = telegram_client
        .post(telegram_api_url)
        .header("Content-Type", "application/json")
        .json(&message_body)
        .send()
        .await;

    match response {
        Ok(res) => {
            if !res.status().is_success(){
                let err_body = res.text().await.unwrap_or_else(|_| String::from("Error Body couldn't be read"));
                println!("Failed to edit message to telegram: {}", err_body);
            }
        },
        Err(e) => {
            println!("Error editing message on Telegram: {}", e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "**¡Exactamente!** La respuesta correcta es A.\n\nAnalicemos el código paso a paso:\n\n1. `let x = 5;` declara una variable `x` y le asigna el valor 5.\n2. `let y = &x;` crea una referencia `y` a `x`.  `y` now holds the memory address of `x`.\n3. `let z = Box::new(y);` crea un `Box` que contiene la referencia `y`.  A `Box` allows you to allocate data on the heap and have a pointer to it on the stack.  In this case, the `Box` contains a reference to a value on the stack (`x`).\n4. `println!(\"{}\", **z);` imprime el valor al que apunta `z`.  Since `z` is a `Box` containing a reference, we need to dereference it twice:\n    * The first `*` dereferences the `Box`, giving us the reference `y`.\n    * The second `*` dereferences the reference `y`, giving us the value of `x`, which is 5.\n\n**Pregunta 4 (Avanzado):**\n\nDada la siguiente enumeración:\n\n```rust\nenum MyEnum {\n    Value(i32),\n    Reference(&'static str),\n}\n```\n\n¿Cómo se accedería al valor entero contenido en una instancia de `MyEnum::Value` utilizando pattern matching?\n\n\nA) `if let MyEnum::Value(value) = my_enum { println!(\"{}\", value); }`\nB) `match my_enum { MyEnum::Value => println!(\"{}\", my_enum), _ => {} }`\nC) `let MyEnum::Value(value) = my_enum; println!(\"{}\", value);`\nD) `println!(\"{}\", my_enum.Value);`\n\n\nElige la respuesta correcta (A, B, C o D).\n\n\n";
    send_telegram_message(text).await;

    Ok(())
}
