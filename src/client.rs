use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

use crate::constants::*;
use crate::crypto::*;

pub async fn start_client() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server_addr = ADDRESS;

    let mut stream = match TcpStream::connect(server_addr).await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            return Err(Box::new(e));
        }
    };
    
    println!("Connected to server at {}", server_addr);

    let message = "Hello, world!";
    stream.write_all(message.as_bytes()).await?;
    println!("Sent: {}", message);

    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;
    if n > 0 {
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("Received: {}", response);
    }

    println!("Closing connection...");
    sleep(Duration::from_secs(2)).await;
    
    Ok(())
}
