mod config;
mod ws;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use tracing::{info, error, warn};
use crate::ws::{Clients, handle_connection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: config::Config = config::Config::new();
    println!("{config:?}");
    tracing_subscriber::fmt::init();

    let addr = format!("{}:{}", config.address, config.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Websocker server is listening on {}", addr);

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, clients.clone()));
    }

    Ok(())
}
