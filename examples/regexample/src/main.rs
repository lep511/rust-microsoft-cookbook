//use regex::Regex;
use reqwest::{Client, Response};
use serde_json::json;

fn replace_raw_escapes(input: &str) -> String {  
    let result = input
        .replace("\n\n\n\n* ", "\n\n")    
        .replace("\n\n\n* ", "\n\n")
        .replace("\n\n* ", "\n\n")
        .replace("\n* ", "\n")
        .replace("**", "*");
    
    result
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
        "parse_mode": "markdown"
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
    let text = "De acuerdo. La respuesta a la pregunta anterior es:\n\nLos **Lifetimes** en Rust son una forma de que el compilador garantice que las referencias sean válidas durante el tiempo que se necesitan.  En esencia, un lifetime es el lapso de tiempo durante el cual una referencia es válida.  Son necesarios para prevenir *dangling pointers*, que son referencias a memoria que ya ha sido liberada.\n\n**Ejemplo:**\n\nImagina una función que devuelve una referencia a una cadena de caracteres creada dentro de la función:\n\n```\nfn devuelve_referencia() -> &str {\n    let s = String::from(\"Hola\");\n    &s // Problema: 's' deja de existir al final de la función\n}\n```\n\nEn este caso, `s` deja de existir al final de la función. Si se devolviera una referencia a `s`, estaríamos intentando usar una referencia a memoria que ya no es válida.  Aquí es donde entran los lifetimes.  El compilador usaría lifetimes para detectar este problema y generar un error de compilación.\n\nUn lifetime se denota con una apóstrofe (`'`) seguido de un nombre (e.g., `'a`). Se utilizan en la firma de la función para indicar la relación entre la duración de las referencias de entrada y la duración de la referencia de salida.\n\n\nAquí va la siguiente pregunta (y última por ahora):\n\nExplica el concepto de *ownership* en Rust y cómo se relaciona con la gestión de memoria.  Describe las reglas básicas de ownership.\n\n\n";
    send_telegram_message(text).await;

    Ok(())
}
