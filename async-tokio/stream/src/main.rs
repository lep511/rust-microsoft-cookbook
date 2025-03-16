use async_stream::stream;
use futures::StreamExt;
use futures::pin_mut;
use tokio::time::{self, Duration, Instant};

// Cambiamos el tipo de retorno a String
fn ticks() -> impl futures::Stream<Item = String> {
    stream! {
        let mut when = Instant::now();
        for i in 0..3 {
            println!("Iniciando tick {}", i + 1);
            
            time::sleep_until(when).await;
            
            // Creamos un mensaje para cada tick
            let mensaje = format!("Mensaje del tick {}: Timestamp: {:?}", 
                                i + 1, 
                                Instant::now());
            
            // yield ahora devuelve un String
            yield mensaje;
            
            println!("Tick {} completado", i + 1);
            when += Duration::from_secs(1);
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Iniciando el programa en: {:?}", Instant::now());
    
    let stream = ticks();
    pin_mut!(stream);
    
    // Ahora recibimos el mensaje en cada tick
    while let Some(mensaje) = stream.next().await {
        println!("Recibido: {}", mensaje);
    }
    
    println!("Stream completado");
}