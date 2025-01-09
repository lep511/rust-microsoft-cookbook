use tokio::net::{TcpListener, TcpStream};
use async_stream::try_stream;
use futures_core::stream::Stream;
use futures_util::StreamExt;
use std::io;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use sha1::{Sha1, Digest};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

fn generate_accept_key(client_key: &str) -> String {
    let mut sha1 = Sha1::new();
    sha1.update(client_key.trim());
    sha1.update("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    BASE64.encode(sha1.finalize())
}

async fn read_websocket_frame(stream: &mut TcpStream) -> io::Result<(Vec<u8>, bool)> {
    let mut header = [0u8; 2];
    stream.read_exact(&mut header).await?;

    let _fin = (header[0] & 0x80) != 0;
    let opcode = header[0] & 0x0F;
    let masked = (header[1] & 0x80) != 0;
    let mut payload_len = (header[1] & 0x7F) as u64;

    if payload_len == 126 {
        let mut ext_len = [0u8; 2];
        stream.read_exact(&mut ext_len).await?;
        payload_len = u16::from_be_bytes(ext_len) as u64;
    } else if payload_len == 127 {
        let mut ext_len = [0u8; 8];
        stream.read_exact(&mut ext_len).await?;
        payload_len = u64::from_be_bytes(ext_len);
    }

    let masking_key = if masked {
        let mut mask = [0u8; 4];
        stream.read_exact(&mut mask).await?;
        Some(mask)
    } else {
        None
    };

    let mut payload = vec![0u8; payload_len as usize];
    stream.read_exact(&mut payload).await?;

    if let Some(mask) = masking_key {
        for i in 0..payload.len() {
            payload[i] ^= mask[i % 4];
        }
    }

    Ok((payload, opcode == 0x8))
}

async fn write_websocket_frame(stream: &mut TcpStream, payload: &[u8]) -> io::Result<()> {
    let payload_len = payload.len();
    let mut header = vec![0u8; 2];
    
    header[0] = 0x81;
    
    if payload_len <= 125 {
        header[1] = payload_len as u8;
    } else if payload_len <= 65535 {
        header[1] = 126;
        header.extend_from_slice(&(payload_len as u16).to_be_bytes());
    } else {
        header[1] = 127;
        header.extend_from_slice(&(payload_len as u64).to_be_bytes());
    }

    stream.write_all(&header).await?;
    stream.write_all(payload).await?;
    stream.flush().await?;
    
    Ok(())
}

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

async fn handle_connection(mut stream: TcpStream) {
    if let Ok(addr) = stream.peer_addr() {
        println!("Handling connection from: {}", addr);
    }
    else {
        println!("Handling connection from an unknown address");
    }
    
    let mut buffer = [0; 1024];
    
    match stream.read(&mut buffer).await {
        Ok(n) => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            println!("Received request:\n{}", request);

            let key = request.lines()
                .find(|line| line.starts_with("Sec-WebSocket-Key:"))
                .and_then(|line| line.split(": ").nth(1))
                .unwrap_or("");

            let accept_key = generate_accept_key(key);

            let response = format!(
                "HTTP/1.1 101 Switching Protocols\r\n\
                 Upgrade: websocket\r\n\
                 Connection: Upgrade\r\n\
                 Sec-WebSocket-Accept: {}\r\n\
                 \r\n",
                accept_key
            );

            match stream.write_all(response.as_bytes()).await {
                Ok(_) => {
                    println!("WebSocket upgrade successful");
                    
                    loop {
                        match read_websocket_frame(&mut stream).await {
                            Ok((payload, is_close)) => {
                                if is_close {
                                    println!("Received close frame");
                                    break;
                                }
                                
                                if let Ok(message) = String::from_utf8(payload) {
                                    println!("Received message: {}", message);
                                    
                                    // Add "Hello" to the response
                                    let response = format!("Hello {}", message);
                                    
                                    // Send the modified response
                                    if let Err(e) = write_websocket_frame(&mut stream, response.as_bytes()).await {
                                        eprintln!("Failed to send response: {}", e);
                                        break;
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to read WebSocket frame: {}", e);
                                break;
                            }
                        }
                    }
                },
                Err(e) => eprintln!("Failed to send response: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
        }
    }

    println!("Connection handled");
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    println!("Server listening on {}", addr);

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
                println!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}