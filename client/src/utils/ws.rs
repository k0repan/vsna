use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream, tungstenite::Message};
use futures_util::{lock::Mutex, StreamExt, SinkExt};
use tracing::error;
use std::sync::Arc;

type WSSink = futures_util::stream::SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, Message>;
type WSRead = futures_util::stream::SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>;

#[derive(Debug, Clone)]
pub struct WebSocketClient {
    write: Arc<Mutex<WSSink>>,
    read: Arc<Mutex<WSRead>>,
}

impl WebSocketClient {
    /// New WebSock with ws_stream
    pub fn new(ws_stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>) -> Self {
        let (write, read) = ws_stream.split();
        Self {
            write: Arc::new(Mutex::new(write)),
            read: Arc::new(Mutex::new(read)),
        }
    }

    pub async fn check_connection(&self) -> bool {
        if let Err(_) = self.get_write().lock().await.send(Message::Ping("cargo is ass".into())).await {
            error!("[!] Err with sending Ping");
            return false;
        }
        match self.get_read().lock().await.next().await {
            Some(Ok(Message::Pong(_))) => true,
            s => {
                println!("[!] Smth happened: {:?}", s);
                false
            },
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
        let mut sender = self.write.lock().await;
        sender.send(Message::Text(text.into())).await?;
        Ok(())
    }
    
    /// Send bins
    pub async fn send_binary(&self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
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
        Ok(())
    }

    pub async fn test_connection(&mut self) -> bool {
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
            true
        }
}