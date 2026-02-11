use tokio_tungstenite::tungstenite::Message;

use crate::{
    config::Config,
    utils::
        file_handler::{
            get_struct_paths_files_with_ignored,
            save_file_server,
            send_file_to_client,
        }
};

/// Handle commands from client and parse it into msg to send back
#[derive(Debug)]
pub struct CommandHandler {
    pub request: String,
    pub body: String,
    config: Config,
}

impl CommandHandler {
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
        save_file_server().await // Decoy. real saving goes to Message::Binary handler
    }
}