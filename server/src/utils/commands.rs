use tokio_tungstenite::tungstenite::Message;
use tracing::info;

use crate::{
    config::Config, utils::{file_handler::{get_struct_paths_files_with_ignored, save_file_server, send_file_to_client}, filepack::FilePacket}
};

#[derive(Debug)]
pub struct CommandHandler {
    pub request: String,
    pub body: String,
    config: Config,
}

impl CommandHandler {
    pub fn new(text: &String, config: Config) -> Self {
        let vec: Vec<&str> = Vec::from_iter(text.split(";"));
        Self {
            request: vec[1].to_string(),
            body: vec[2].to_string(),
            config: config,
        }
    }

    pub async fn parse_text_to_command(&self) -> Vec<Option<Message>> {
        self.parse_command().await
    }

    pub async fn parse_command(&self) -> Vec<Option<Message>> {
        match self.request.as_str() {
            "DOWNLOAD_FILES" => self.download_files_server().await,
            "SHOW_PATH" => self.show_path_server().await,
            "SEND_FILES" => self.send_files_server().await,
            _ => vec![None],
        }
    }

    async fn download_files_server(&self) -> Vec<Option<Message>> {
        send_file_to_client(&self.config, &self.body).await
    }

    async fn show_path_server(&self) -> Vec<Option<Message>> {
        vec![Some(Message::Text(get_struct_paths_files_with_ignored(&self.config, self.body.clone()).await.into()))]
    }

    async fn send_files_server(&self) -> Vec<Option<Message>> {
        save_file_server(&self.config, &self.body).await
    }
}

pub async fn save_file_bytes_server(config: &Config, bytes: &[u8]) -> u64 {
    let packet: FilePacket = FilePacket::from_bytes(&bytes).expect("[!] Err with unpack from bytes");
    info!("Received file: {}", &packet.filename);
    
    let _ = packet.save(&config.server_path).await.expect("[!] Err with saving file");
    packet.get_size()
}