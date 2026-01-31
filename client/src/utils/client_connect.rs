use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use tracing::info;
use std::{io, net::TcpStream};
use crate::{
    config::Config,
    utils::client_cli::client_cli,
};


pub async fn check_connection(url: &String) -> bool {
    TcpStream::connect(url).is_ok()
}

pub async fn client_connect(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut ip: String = String::new();
    println!("[>] Input IP addr:");
    io::stdin()
        .read_line(&mut ip)
        .expect("[!] Err with reading IP");

    let mut port: String = String::new();
    println!("[>] Input Port:");
    io::stdin()
        .read_line(&mut port)
        .expect("[!] Err with reading Port");
    let ip: &str = ip.trim();
    let port: &str = port.trim();

    let addr: String = if ip.is_empty() || port.is_empty() {
        config.get_addr()
    } else {
        format!("{}:{}", ip, port)
    };

    let url: String = format!("ws://{}", addr);
    let (ws_stream, _) = connect_async(url).await?;
    info!("Connected to WebSocket server");

    let (mut write, mut read) = ws_stream.split();
    
    // Spawn a task to handle incoming messages
    let read_handle = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    info!("Received: {}", text);
                }

                Ok(Message::Binary(bin)) => {
                    info!("Received {} bytes", bin.len());
                }

                Ok(Message::Close(_)) => {
                    info!("Server closed connection");
                    break;
                }
                _ => {}
            }
        }
    });

    // EXAMPLE: Send a text
    write.send(Message::Text("Hello, World!".to_string().into())).await?;

    // EXAMPLE: Send a JSON (can be rewritten for files)
    let json_msg = serde_json::json!({
        "type": "info",
        "text": "Hello, World!"
    });
    write.send(Message::Text(json_msg.to_string().into())).await?;

    client_cli(config).await;

    write.send(Message::Close(None)).await?;
    read_handle.await?;
    Ok(())
}