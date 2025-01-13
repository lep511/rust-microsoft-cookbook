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
    let text = "Sí, se puede decir que los closures en Rust son similares a las funciones lambda en Python en cuanto a su funcionalidad de definir funciones anónimas en línea. Ambos permiten crear funciones sin nombre que se pueden pasar como argumentos a otras funciones o utilizarse en expresiones.\n\nSin embargo, hay algunas diferencias importantes:\n\n* **Captura de variables:** Los closures en Rust tienen la capacidad de capturar variables del entorno que los rodea, mientras que las funciones lambda en Python tienen restricciones en la captura de variables (solo pueden acceder a variables globales o variables en el ámbito de la función que las encierra, pero no pueden modificarlas).  En Rust, la forma en que un closure captura variables se define mediante los traits `Fn`, `FnMut` y `FnOnce`.\n\n* **Inferencia de tipos:** Rust tiene un sistema de inferencia de tipos más potente que Python, lo que permite que los closures tengan tipos inferidos para sus parámetros y valor de retorno.  En Python, los tipos generalmente se infieren en tiempo de ejecución, mientras que en Rust se infieren en tiempo de compilación.\n\n* **'Lifetimes':**  En Rust, los closures pueden tener 'lifetimes' asociados para garantizar la seguridad de la memoria cuando se capturan referencias. Este concepto no existe en Python.\n\n\n\nEn resumen, si bien comparten la idea de funciones anónimas, los closures en Rust son más poderosos y flexibles debido a su capacidad de capturar variables del entorno, la inferencia de tipos y el manejo de 'lifetimes' para la seguridad de la memoria.\n\n\nAhora que hemos aclarado esto, ¿te gustaría que volvamos a la **pregunta 11** sobre mutabilidad interior y exterior, o prefieres explorar más a fondo algún otro tema?\n";
    send_telegram_message(text).await;

    Ok(())
}
