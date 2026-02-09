use std::io;
use tokio_tungstenite::tungstenite::{Bytes, Message};

use crate::utils::filepack::FilePacket;

pub fn read_string() -> String {
    let mut read_str: String = String::new();
    io::stdin()
        .read_line(&mut read_str)
        .expect("[!] Err with readline");

    read_str.trim().to_string()
}

pub async fn receive_file_from_server(path: &String, bytes: Bytes) {
    let packet: FilePacket = FilePacket::from_bytes(&bytes).expect("[!] Err with unpack from bytes");
    println!("[=] Received file: {}", &packet.filename);
    
    match packet.save(path).await {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn get_bytes_of_file(path: &String, filename: &String) -> Option<Message> {
    let file_loc: String = format!("{}{}", &path, filename);
    println!("[=] Filename requested: {}", file_loc);

    let packet: FilePacket = FilePacket::from_file(&file_loc).await.expect("[!] Err with pack bytes");
    let bytes: Vec<u8> = packet.to_bytes().expect("[!] Err with convert to bytes");
    Some(Message::Binary(bytes.into()))
}