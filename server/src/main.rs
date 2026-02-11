mod config;
mod utils;

use std::{sync::Arc, collections::HashMap};
use tokio::{net::TcpListener, sync::RwLock};
use tracing::info;
use crate::{
    config::Config, utils::ws::{Clients, handle_connection}
};

/// Run WS server
async fn start_server(config: &Config) {
    let addr = config.get_addr();
    let listener = TcpListener::bind(&addr).await;
    info!("WebSocket server is listening on {}", addr);

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    while let Ok((stream, addr)) = listener
        .as_ref()
        .expect("[!] Err with Listener")
        .accept()
        .await
    {
        tokio::spawn(handle_connection(stream, addr, clients.clone(), config.clone()));
    }
}


#[tokio::main]
async fn main() {
    let config: Config = Config::new();
    println!("{config:?}");
    tracing_subscriber::fmt::init();

    start_server(&config).await;
}
