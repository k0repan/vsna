use std::io;
use futures_util::StreamExt;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    config::Config,
    utils::{
        ws::WebSocketClient,
        file_handler::receive_file_from_server,
        file_handler::get_bytes_of_file,
    },
};


pub async fn client_cli(config: &Config, ws_stream: WebSocketClient) {
    loop {
        println!("");
        println!("[0] Exit");
        println!("[1] Show server path");
        println!("[2] Download files");
        println!("[3] Send files");
        println!("[4] Check connection");

        if !ws_stream.check_connection().await {
            println!("[!] Went out from Client CLI loop");
            break;
        }

        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("[!] Err with read choice");

        let choice: &str = choice.trim();
        
        match choice {
            "0" => break,
            "1" => show_path_client(&ws_stream).await,
            "2" => download_files_client(&config.client_path, &ws_stream).await,
            "3" => send_files_client(&config.client_path, &ws_stream).await,
            "4" => {
                if ws_stream.check_connection().await {
                    println!("[=] Connection is successfull");
                } else {
                    println!("[!] Err with connection");
                    break;
                }
            },
            _ => println!("[!] Unknown command"),
        }
    }
}


async fn show_path_client(ws_stream: &WebSocketClient) {
    println!("[>] Input path:");
    let mut request_path: String = String::new();
    io::stdin()
        .read_line(&mut request_path)
        .expect("[!] Err with readline");

    let request_path: &str = request_path.trim();

    if let Err(e) = ws_stream.send_text(format!("cmd;SHOW_PATH;{}", request_path)).await {
        println!("[!] Failed to send: {}", e);
        return;
    }
    
    // Response
    while let Some(msg) = ws_stream.get_read().lock().await.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("[=] Files:\n{}", text);
                break;
            },
            Ok(Message::Pong(_)) => continue,
            Ok(Message::Close(_)) => break,
            s => {
                println!("[!] Error: {:?}", s);
                break;
            },
        }
    }
    
}

async fn download_files_client(client_path: &String, ws_stream: &WebSocketClient) {
    println!("[>] Input file(s)/path name to download:");
    let mut request_files: String = String::new();
    io::stdin()
        .read_line(&mut request_files)
        .expect("[!] Err with readline");

    let request_files: &str = request_files.trim();

    if let Err(e) = ws_stream.send_text(format!("cmd;DOWNLOAD_FILES;{}", request_files)).await {
        println!("[!] Failed to send: {}", e);
        return;
    }
    
    //Response
    while let Some(msg) = ws_stream.get_read().lock().await.next().await {
        match msg {
            Ok(Message::Binary(bytes)) => {
                println!("\n[=] Downloaded: {} B", bytes.len());
                receive_file_from_server(client_path, bytes).await;
                break;
            },
            Ok(Message::Pong(_)) => continue,
            Ok(Message::Close(_)) => break,
            s => {
                println!("[!] Error: {:?}", s);
                break;
            },
        }
    }
}

async fn send_files_client(client_path: &String, ws_stream: &WebSocketClient) {
    println!("[>] Input file(s)/path name to send:");
    let mut client_files: String = String::new();
    io::stdin()
        .read_line(&mut client_files)
        .expect("[!] Err with readline");

    let client_files: &str = client_files.trim();

    if let Err(e) = ws_stream.send_text(format!("cmd;SEND_FILES;{}", client_files)).await {
        println!("[!] Failed to send: {}", e);
        return;
    }
    for client_file in client_files.split(" ") {
        match get_bytes_of_file(&client_path, &client_file.to_string()).await {
            Some(msg) => {
                if let Err(e) = ws_stream.send_binary(msg.into_data().into()).await {
                    println!("[!] Failed to send: {}", e);
                    continue;
                }
            },
            None => continue,
        }
        
    }
    
    //Response
    while let Some(msg) = ws_stream.get_read().lock().await.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("\n[=] Sended: {} B", text);
                break;
            },
            Ok(Message::Pong(_)) => continue,
            Ok(Message::Close(_)) => break,
            s => {
                println!("[!] Error: {:?}", s);
                break;
        },
        }
    }
}