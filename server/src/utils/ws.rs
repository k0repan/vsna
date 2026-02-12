use std::{net::SocketAddr, sync::Arc, collections::HashMap};
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{accept_async, tungstenite::{Message, Utf8Bytes},};
use futures_util::{SinkExt, StreamExt};
use tracing::{error, info, warn};

use crate::{
    config::Config,
    utils::{
        command_handler::{
            ServerCommandHandler,
            ServerMsg
        },
        file_handler::{convert_msg_to_close, save_file_bytes_server},
    }
};

pub type Clients = Arc<RwLock<HashMap<SocketAddr, tokio::sync::mpsc::UnboundedSender<Message>>>>;

/// Main server handler. Add clients and works with streams (write, read)
pub async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    clients: Clients,
    config: Config,
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
                info!("Received text from {}: {}", addr, &text);
                if text.len() > 3 && text.trim().to_string().starts_with("cmd;") {
                    match handle_text_request(&clients, addr, text, &config).await {
                        Ok(_) => (),
                        Err(_) => break,
                    };
                }
            },
            Ok(Message::Binary(bin)) => {
                info!("Received {} bytes from {}", bin.len(), addr);
                broadcast_message(&clients, save_file_bytes_server(&config, &bin).await, addr).await;
            }
            Ok(Message::Close(_)) => {
                info!("Client {} disconnected", addr);
                break;
            }
            Ok(Message::Ping(data)) => {
                if let Some(tx) = clients.read().await.get(&addr) {
                    match tx.send(Message::Pong(data)) {
                        Ok(_) => {},
                        Err(e) => {
                            error!("Err with send Pong: {}", e);
                            break;
                        }
                    }
                }
            }
            Ok(Message::Pong(_)) => {}
            Ok(Message::Frame(_)) => todo!(),
            Err(e) => {
                error!("Websocket error for {}: {}", addr, e);
                break;
            }
        }
    }

    send_task.abort();
    clients.write().await.remove(&addr);
    info!("Client {} removed", addr);
}

/// Send msg to sender
async fn broadcast_message(clients: &Clients, msg: Message, sender: SocketAddr) {
    let clients = clients.read().await;
    for (addr, tx) in clients.iter() {
        if *addr == sender {
            tx.send(msg.clone()).ok();
        }
    }
}

/// Main command parser. Get all responses from server and pack it to Vec of msgs. Then send it
async fn handle_text_request(clients: &Clients, addr: SocketAddr, text: Utf8Bytes, config: &Config) -> Result<(), ()> {
    let cmd: ServerCommandHandler = ServerCommandHandler::new(&text.to_string(), config.clone());
    let vec_msg: ServerMsg = cmd.parse_command().await;
    if vec_msg.len() > 0 as usize {
        if vec_msg.iter().all(|el| el.is_none()) {
            warn!("All files are oversized... Wtf bro?");
            broadcast_message(&clients, convert_msg_to_close("All messages skipped.".to_string()).unwrap(), addr).await;
        } else {
            for msg in vec_msg {
                match msg {
                    Some(response) => {
                        broadcast_message(&clients, response, addr).await;
                    },
                    None => {
                        warn!("Found oversized file! File skipped...");
                    },
                }
            }
        }
        
    } else {
        error!("Err with cmd.parse_text_to_command occured!");
        return Err(());
    }
    Ok(())
}