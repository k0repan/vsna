use tokio_tungstenite::tungstenite::Message;

use crate::{
    config::Config, utils::{
        file_handler::{
            get_bytes_of_file,
            read_string,
            receive_file_from_server,
        },
        ws::WebSocketClient,
    }
};

#[derive(Debug)]
pub struct ClientCommandHandler<'a> {
    config: &'a Config,
    ws_stream: WebSocketClient,
}

impl<'a> ClientCommandHandler<'a> {
    pub fn new(_config: &'a Config, _ws_stream: WebSocketClient) -> Self {
        Self {
            config: _config,
            ws_stream: _ws_stream,
        }
    }

    /// Get server path with all files by all layers
    pub async fn show_path_request(&self) {
        println!("[>] Input path:");
        let request_path: &str = &read_string();
        
        if let Err(e) = self.ws_stream.send_text(format!("cmd;SHOW_PATH;{}", request_path)).await {
            println!("[!] Failed to send: {}", e);
            return;
        }
        
        // Read response
        while let Some(msg) = self.ws_stream.get_read().await {
            match msg {
                Ok(Message::Text(text)) => {
                    println!("[=] Files:\n{}", text);
                    break;
                },
                Ok(Message::Pong(_)) => continue,
                Ok(Message::Close(close)) => {
                    if let Some(some) = close {
                        println!("[!] CloseFrame: {}", some);
                    }
                    break;
                },
                e => {
                    println!("[!] Error: {:?}", e);
                    break;
                },
            }
        }    
    }

    /// Download files bytes from server
    /// Support ignoring files, fmts and paths
    pub async fn download_files_request(&self) {
        println!("[>] Input file(s)/path name to download:");
        let request_files: &str = &read_string();

        if let Err(e) = self.ws_stream.send_text(format!("cmd;DOWNLOAD_FILES;{}", request_files)).await {
            println!("[!] Failed to send: {}", e);
            return;
        }

        //Read response
        while let Some(msg) = self.ws_stream.get_read().await {
            match msg {
                Ok(Message::Binary(bytes)) => {
                    if *bytes == *b"cargo is ass" {
                        println!("[=] Download finished.");
                        break;
                    } else {
                        receive_file_from_server(&self.config.client_path, bytes).await;
                    }
                },
                Ok(Message::Pong(_)) => continue,
                Ok(Message::Close(close)) => {
                    if let Some(some) = close {
                        println!("[!] CloseFrame: {}", some);
                    }
                    break;
                },
                e => {
                    println!("[!] Error: {:?}", e);
                    break;
                },
            }
        }
    }

    /// Send files from client to server
    pub async fn send_files_request(&self) {
        println!("[>] Input file(s)/path name to send:");
        let client_files: &str = &read_string();

        if let Err(e) = self.ws_stream.send_text(format!("cmd;SEND_FILES;{}", client_files)).await {
            println!("[!] Failed to send: {}", e);
            return;
        }
        for client_file in client_files.split_whitespace() {
            match get_bytes_of_file(&self.config.client_path, &client_file.to_string()).await {
                Some(msg) => {
                    if let Err(e) = self.ws_stream.send_binary(msg.into_data().into()).await {
                        println!("[!] Failed to send: {}", e);
                        continue;
                    }
                },
                None => continue,
            }
        }

        //Read response
        while let Some(msg) = self.ws_stream.get_read().await {
            match msg {
                Ok(Message::Text(text)) => {
                    println!("\n[=] Sended: {} B", text);
                    break;
                },
                Ok(Message::Pong(_)) => continue,
                Ok(Message::Close(close)) => {
                    if let Some(some) = close {
                        println!("[!] CloseFrame: {}", some);
                    }
                    break;
                },
                e => {
                    println!("[!] Error: {:?}", e);
                    break;
            },
            }
        }
    }

    pub async fn check_connection(&self) -> Result<&str, &str> {
        if self.ws_stream.check_connection().await {
            Ok("[=] Connection is successfull")
        } else {
            Err("[!] Err with connection")
        }
    }
}