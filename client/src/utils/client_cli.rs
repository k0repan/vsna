use std::io;
use crate::{
    config::Config,
    utils::{
        client_connect::check_connection,
        ws::WebSocketClient,
    },
};


pub async fn client_cli(config: &Config, ws_stream: WebSocketClient) {
    loop {
        println!("");
        println!("[0] Exit");
        println!("[1] Show path");
        println!("[2] Download files");
        println!("[3] Send files");
        println!("[4] Check connection");

        if !check_connection(&ws_stream).await {
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
                if check_connection(&ws_stream).await {
                    println!("[=] Connection is successfull");
                } else {
                    println!("[!] Err with connection");
                }
            },
            _ => println!("[!] Unknown command"),
        }
    }
}


async fn show_path_client(ws_stream: &WebSocketClient) {
    if !check_connection(&ws_stream).await {
        return;
    }

    let mut request_path: String = String::new();
    io::stdin()
        .read_line(&mut request_path)
        .expect("[!] Err with readline");

    let request_path: &str = request_path.trim();

    if let Err(e) = ws_stream.send_text(format!("GET_PATHS {}", request_path)).await {
        println!("[!] Failed to send: {}", e);
        return;
    }
    
    // Response
    //match read.next().await {
    //    Some(Ok(Message::Text(text))) => println!("\n[=] Files:\n{}", text),
    //    Some(Err(e)) => println!("[!] Error: {}", e),
    //    _ => println!("[!] No response"),
    //}
}

async fn download_files_client(client_path: &String, ws_stream: &WebSocketClient) {
    if !check_connection(&ws_stream).await {
        return;
    }

    let mut request_files: String = String::new();
    io::stdin()
        .read_line(&mut request_files)
        .expect("[!] Err with readline");

    let request_files: &str = request_files.trim();

    if let Err(e) = ws_stream.send_text(format!("DOWNLOAD_FILES {}", request_files)).await {
        println!("[!] Failed to send: {}", e);
        return;
    }
    
    // Response
    //match read.next().await {
    //    Some(Ok(Message::Text(text))) => println!("\n[=] Downloaded:\n{}", text),
    //    Some(Err(e)) => println!("[!] Error: {}", e),
    //    _ => println!("[!] No response"),
    //}
}

async fn send_files_client(client_path: &String, ws_stream: &WebSocketClient) {
    if !check_connection(&ws_stream).await {
        return;
    }

    let mut client_files: String = String::new();
    io::stdin()
        .read_line(&mut client_files)
        .expect("[!] Err with readline");

    let client_files: &str = client_files.trim();

    if let Err(e) = ws_stream.send_text(format!("SEND_FILES {}", client_files)).await {
        println!("[!] Failed to send: {}", e);
        return;
    }

    //write sending bytes of files
    
    // Response
    //match read.next().await {
    //    Some(Ok(Message::Text(text))) => println!("\n[=] Downloaded:\n{}", text),
    //    Some(Err(e)) => println!("[!] Error: {}", e),
    //    _ => println!("[!] No response"),
    //}
}