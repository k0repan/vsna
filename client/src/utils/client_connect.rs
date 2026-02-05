use std::io;
use tracing::info;
use crate::{
    config::Config,
    utils::client_cli::client_cli,
    utils::ws::WebSocketClient,
};

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

    ws_client.test_connection().await;
    client_cli(config, ws_client).await;
}