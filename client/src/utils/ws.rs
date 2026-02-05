use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream, tungstenite::Message};
use futures_util::{lock::Mutex, StreamExt, SinkExt};
use std::sync::Arc;
use tracing::info;

type WSSink = futures_util::stream::SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, Message>;
type WSRead = futures_util::stream::SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>;

#[derive(Debug, Clone)]
pub struct WebSocketClient {
    write: Arc<Mutex<WSSink>>,
    read: Arc<Mutex<WSRead>>,
    pub is_connected: bool,
}

impl WebSocketClient {
    /// New WebSock with ws_stream
    pub fn new(ws_stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>) -> Self {
        let (write, read) = ws_stream.split();
        Self {
            write: Arc::new(Mutex::new(write)),
            read: Arc::new(Mutex::new(read)),
            is_connected: true,
        }
    }

    pub fn get_read(&self) -> Arc<Mutex<WSRead>> {
        self.read.clone()
    }

    pub fn get_write(&self) -> Arc<Mutex<WSSink>> {
        self.write.clone()
    }

    /// Connect WebSock to url
    pub async fn connect(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (ws_stream, _) = connect_async(url).await?;
        Ok(Self::new(ws_stream))
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
        let json: String = serde_json::to_string(data)?;
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
    pub async fn test_read_handle(&mut self) {
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

        read_handler.await.expect("[!] Err with read handler");
    }

    pub async fn test_connection(&mut self) -> bool {
        if !self.is_connected {
            false
        } else {
            match self.send_text("Hello, string!".to_string()).await {
                Ok(_) => (),
                Err(_) => return false,
            };

            let test_json = serde_json::json!({
                "type": "info",
                "data": "Hello, json"
            });
            
            match self.send_json(&test_json).await {
                Ok(_) => (),
                Err(_) => return false,
            };
            //self.test_read_handle().await;
            true
        }
    }
}