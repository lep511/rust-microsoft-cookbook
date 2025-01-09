use tokio::net::{TcpListener, TcpStream};
use async_stream::try_stream;
use futures_core::stream::Stream;
use futures_util::StreamExt;
use std::io;
use std::net::SocketAddr;

fn bind_and_accept(addr: SocketAddr) 
    -> impl Stream<Item = io::Result<TcpStream>> 
{
    try_stream! {
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (stream, addr) = listener.accept().await?;
            println!("received on {:?}", addr);
            yield stream;
        }
    }
}

async fn handle_connection(stream: TcpStream) {
    if let Ok(addr) = stream.peer_addr() {
        println!("Handling connection from: {}", addr);
    }
    
    // Add your connection handling logic here
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    println!("Server listening on {}", addr);

    // Pin the stream to make it Unpin
    let incoming = bind_and_accept(addr);
    let mut incoming = Box::pin(incoming);

    while let Some(stream) = incoming.next().await {
        match stream {
            Ok(stream) => {
                tokio::spawn(async move {
                    handle_connection(stream).await;
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}