use tokio_tungstenite::tungstenite::Message;

use crate::{
    config::Config,
    utils::file_handler::{
        convert_msg_to_close,
        get_server_path_str,
        save_file_server,
        send_file_to_client,
    },
};

pub type ServerMsg = Vec<Option<Message>>;

/// Handle commands from client and parse it into msg to send back
#[derive(Debug)]
pub struct ServerCommandHandler {
    request: String,
    body: String,
    config: Config,
}

impl ServerCommandHandler {
    /// Read txt str as client cmd
    pub fn new(text: &String, config: Config) -> Self {
        let vec: Vec<&str> = Vec::from_iter(text.split(";"));
        Self {
            request: vec[1].to_string(),
            body: vec[2].to_string(),
            config: config,
        }
    }

    /// Parse cmd as pattern
    /// TODO: encapsulate out cmd patterns as Enum
    pub async fn parse_command(&self) -> ServerMsg {
        match self.request.as_str() {
            "DOWNLOAD_FILES" => self.download_files_server().await,
            "SHOW_PATH" => self.show_path_server().await,
            "SEND_FILES" => self.send_files_server().await,
            _ => vec![convert_msg_to_close("No command found".to_string())],
        }
    }

    async fn download_files_server(&self) -> ServerMsg {
        send_file_to_client(&self.config, &self.body).await
    }

    async fn show_path_server(&self) -> ServerMsg {
        get_server_path_str(&self.config, &self.body).await
    }

    async fn send_files_server(&self) -> ServerMsg {
        save_file_server().await // Decoy. Real saving goes to file_handler/save_file_bytes_server
    }
}