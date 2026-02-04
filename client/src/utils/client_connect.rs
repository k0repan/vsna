use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream, tungstenite::Message};
use futures_util::{lock::Mutex, StreamExt, SinkExt};
use std::{io, sync::Arc};
use tracing::info;
use crate::{
    config::Config,
    utils::client_cli::client_cli,
};

type WSSink = futures_util::stream::SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, Message>;
type WSRead = futures_util::stream::SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>;

#[derive(Debug, Clone)]
pub struct WebSocketClient {
    write: Arc<Mutex<WSSink>>,
    read: Arc<Mutex<WSRead>>,
    url: String,
    is_connected: bool,
}

impl WebSocketClient {
    /// New WebSock with ws_stream
    pub fn new(ws_stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, url: &String) -> Self {
        let (write, read) = ws_stream.split();
        Self {
            write: Arc::new(Mutex::new(write)),
            read: Arc::new(Mutex::new(read)),
            url: url.to_string(),
            is_connected: true,
        }
    }

    /// Connect WebSock to url
    pub async fn connect(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (ws_stream, _) = connect_async(url).await?;
        Ok(Self::new(ws_stream, &url.to_string()))
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    /// Send txt
    pub async fn send_text(&self, text: String) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_connected {
            return Err("Not connected".into());
        }
        
        let mut sender = self.write.lock().await;
        sender.send(Message::Text(text.into())).await?;
        Ok(())
    }
    
    /// Send bins
    pub async fn send_binary(&self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_connected {
            return Err("Not connected".into());
        }
        
        let mut sender = self.write.lock().await;
        sender.send(Message::Binary(data.try_into().unwrap())).await?;
        Ok(())
    }
    
    /// Send JSON
    pub async fn send_json<T: serde::Serialize>(&self, data: &T) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(data)?;
        self.send_text(json).await?;
        Ok(())
    }
    
    /// Close WebSock
    pub async fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut sender = self.write.lock().await;
        sender.send(Message::Close(None)).await?;
        self.is_connected = false;
        Ok(())
    }
    
    /// Standard read handle (blocks until reader finishes)
    pub async fn std_read_handle(&mut self) {
        let read_clone: Arc<Mutex<WSRead>> = Arc::clone(&self.read);
        
        let read_handler = tokio::spawn(async move {
            while let Some(msg) = read_clone.lock().await.next().await {
                match msg {
                    Ok(Message::Text(text)) => info!("Received: {}", text),
                    Ok(Message::Binary(bin)) => info!("Received {} bytes", bin.len()),
                    Ok(Message::Close(_)) => {
                        info!("Server closed connection");
                        break;
                    }
                    _ => {}
                }
            }
        });

        read_handler.await.unwrap();
    }
}

pub async fn check_connection(ws_stream: &WebSocketClient) -> bool {
    ws_stream.is_connected
}

pub async fn client_connect(config: &Config) {
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
    let mut ws_client = WebSocketClient::connect(&url).await.unwrap();
    info!("Connected to WebSocket server");

    test_connection(&mut ws_client).await;
    client_cli(config, ws_client).await;
}


async fn test_connection(ws_stream: &mut WebSocketClient) -> bool {
    if !ws_stream.is_connected {
        false
    } else {
        ws_stream.send_text("Hello, string!".to_string())
            .await
            .unwrap();

        let test_json = serde_json::json!({
            "type": "info",
            "data": "Hello, json"
        });
        ws_stream.send_json(&test_json)
            .await
            .unwrap();
        let _ = &ws_stream.std_read_handle().await;
        true
    }
}