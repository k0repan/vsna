use std::{net::SocketAddr, sync::Arc, collections::HashMap};
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use tracing::{info, error};

pub type Clients = Arc<RwLock<HashMap<SocketAddr, tokio::sync::mpsc::UnboundedSender<Message>>>>;

pub async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    clients: Clients
) {
    info!("New Websocket connection from {}", addr);

    let ws_stream = accept_async(stream).await.expect("[!] Err with accept_async");
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // Store the client
    clients.write().await.insert(addr, tx);

    // Spawn task to handle outgoing messages
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming message
    while let Some(msg) = ws_receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received text from {}: {}", addr, text);
                broadcast_message(&clients, Message::Text(text), addr).await;
            }
            Ok(Message::Binary(bin)) => {
                info!("Received {} bytes from {}", bin.len(), addr);
                broadcast_message(&clients, Message::Binary(bin), addr).await;
            }
            Ok(Message::Close(_)) => {
                info!("Client {} disconnected", addr);
                break;
            }
            Ok(Message::Ping(data)) => {
                if let Some(tx) = clients.read().await.get(&addr) {
                    tx.send(Message::Pong(data)).ok();
                }
            }
            Ok(Message::Pong(_)) => {}
            Ok(Message::Frame(_)) => todo!(),
            Err(e) => {
                error!("Websocker error for {}: {}", addr, e);
                break;
            }
        }
    }

    send_task.abort();
    clients.write().await.remove(&addr);
    info!("Client {} removed", addr);
}


async fn broadcast_message(clients: &Clients, msg: Message, sender: SocketAddr) {
    let clients = clients.read().await;
    for (addr, tx) in clients.iter() {
        if *addr != sender {
            tx.send(msg.clone()).ok();
        }
    }
}