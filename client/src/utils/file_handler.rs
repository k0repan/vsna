use std::io;
use tokio_tungstenite::tungstenite::{Bytes, Message};

use crate::utils::filepack::FilePacket;

/// CLI input stream reader
pub fn read_string() -> String {
    let mut read_str: String = String::new();
    io::stdin()
        .read_line(&mut read_str)
        .expect("[!] Err with readline");

    read_str.trim().to_string()
}

/// Receive and save file from bytes
pub async fn receive_file_from_server(path: &String, bytes: Bytes) {
    match FilePacket::from_bytes(&bytes) {
        Err(e) => println!("[!] Err with unpack from bytes: {}", e),
        Ok(packet) => {
            println!("[=] Received file: {}", &packet.filename);
    
            match packet.save(path).await {
                Ok(size) => {
                    println!("[=] Downloaded: {}", size);
                },
                Err(e) => {
                    println!("{:?}", e);
                },
            }
        }
    }
}

/// Parse file from client_path to msg bytes
pub async fn get_bytes_of_file(path: &String, filename: &String) -> Option<Message> {
    let file_loc: String = format!("{}{}", &path, filename);
    println!("[=] Filename requested: {}", file_loc);

    if let Ok(packet) = FilePacket::from_file(&file_loc).await {
        if let Ok(bytes) = packet.to_bytes() {
            Some(Message::Binary(bytes.into()))
        } else {
            println!("[!] Err with convert to bytes");
            None
        }
    } else {
        println!("[!] Err with pack bytes");
        None
    }
}