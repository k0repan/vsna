mod config;
mod utils;

use std::{collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};
use tracing::{error, info};
use crate::{
    config::Config, utils::ws::{Clients, handle_connection}
};

/// Run WS server
async fn start_server(config: Config) {
    let addr = config.get_addr();
    let listener = TcpListener::bind(&addr).await;
    info!("WebSocket server is listening on {}", addr);

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    while let Ok((stream, addr)) = listener
        .as_ref()
        .unwrap()
        .accept()
        .await
    {
        tokio::spawn(handle_connection(stream, addr, clients.clone(), config.clone()));
    }
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Ok(config) = Config::new() {
        println!("{config:?}");
        start_server(config).await;
    } else {
        error!("Error with config");
    }
}
