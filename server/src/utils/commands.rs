use tokio_tungstenite::tungstenite::Message;

use crate::{
    utils::file_handler::_read_path,
    utils::file_handler::send_file_to_client,
    config::Config,
};

#[derive(Debug)]
pub struct Command {
    pub request: String,
    pub body: String,
    config: Config,
}

impl Command {
    pub fn new(text: &String, config: Config) -> Self {
        let vec: Vec<&str> = Vec::from_iter(text.split(";"));
        Self {
            request: vec[1].to_string(),
            body: vec[2].to_string(),
            config: config,
        }
    }

    pub async fn parse_text_to_command(&self) -> Option<Message> {
        self.parse_command().await
    }

    pub async fn parse_command(&self) -> Option<Message> {
        match self.request.as_str() {
            "DOWNLOAD_FILES" => self.download_files_server().await,
            "SHOW_PATH" => self.show_path_server().await,
            "SEND_FILES" => self.send_files_server().await,
            _ => None,
        }
    }

    async fn download_files_server(&self) -> Option<Message> {
        send_file_to_client(&self.config, &self.body).await
    }

    async fn show_path_server(&self) -> Option<Message> {
        Some(Message::Text(_read_path(&self.config, self.body.clone()).await.into()))
    }

    async fn send_files_server(&self) -> Option<Message> {
        Some(Message::Binary("Send".into()))
    }
}