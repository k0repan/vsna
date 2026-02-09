use crate::{
    config::Config,
    utils::{
        client_cli::client_cli,
        file_handler::read_string,
        ws::WebSocketClient,
    },
};

pub async fn client_connect(config: &Config) {
    println!("[>] Input IP addr:");
    let ip: &str = &read_string();

    println!("[>] Input Port:");
    let port: &str = &read_string();

    let addr: String = if ip.is_empty() || port.is_empty() {
        config.get_addr()
    } else {
        format!("{}:{}", ip, port)
    };

    let url: String = format!("ws://{}", addr);
    let mut ws_client = WebSocketClient::connect(&url).await.expect("[!] Err with WebSocket connection");
    println!("[=] Connected to WebSocket server");

    if ws_client.test_connection().await {
        println!("[=] Test connection successfull");
    } else {
        println!("[!] Err with test connection");
        return;
    }
    client_cli(config, ws_client).await;
}