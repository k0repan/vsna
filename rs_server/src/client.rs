use std::{fs, io};
use axum::Error;

use crate::host::Message;
use crate::dir_handler::read_path_as_client;
use crate::config::CONFIG;

async fn connect_to_network() -> String {
    // Connecting to VPN via name and pwd
    println!("[>] Input IP of the server (localhost, 192.0.12.3): ");
    let mut server_ip: String = String::new();
    io::stdin().read_line(&mut server_ip).expect("[!] Cannot read IP");
    let server_ip: &str = server_ip.trim();
    
    format!("http://{}:5555", server_ip) // return url
}

async fn download_file(url: &String) -> Result<(), String> {
    read_path_as_client().await;
    // input file names & copy files to client device
    Ok(())
}

async fn read_file() {
    println!("[=] You are in directory {}", &CONFIG.usr_path);
    println!("[>] Input file name: ");
    let mut input_file: String = String::new();
    io::stdin()
        .read_line(&mut input_file)
        .expect("[!] Cannot read filename");

    let input_file: &str = input_file.trim();

    let mut filename: String = (&CONFIG.usr_path).to_string();
    filename.push_str(&input_file);

    match fs::read_to_string(&filename) {
        Ok(content) => {
            println!("{}:\n{}", filename, content);
        }
        Err(e) => {
            eprintln!("[!] Cannot read '{}': {}", filename, e);
        }
    }
}

async fn _send_message(url: &String) -> Result<(), String> {
    println!("[>] Input name: ");
    let mut from: String = String::new();
    io::stdin()
        .read_line(&mut from)
        .expect("[!] Cannot read name");
    
    println!("[>] Input message: ");
    let mut text: String = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("[!] Cannot read message");
    
    let message: Message = Message {
        text: text.trim().to_string(),
        from: from.trim().to_string(),
    };
    
    let client: reqwest::Client = reqwest::Client::new();
    match client.post(&format!("{}/send", url))
        .json(&message) 
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                let response_msg: Message = response.json().await.expect("");
                println!("[=] Server response: {}", response_msg.text);
            } else {
                println!("[!] {}", response.status());
            }
        }
        Err(e) => println!("[!] {e}"),
    }
    
    Ok(())
}

pub async fn check_connection(url: &String) -> Result<(), Error> {
    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {
                let text: String = response.text().await.unwrap();
                println!("[=] Connnect successful! Response: {}", text);
                Ok(())
            } else {
                println!("[!] {}", response.status());
                return Err(axum::Error::new(""));
            }
        }
        Err(e) => {
            println!("[!] {e}");
            return Err(axum::Error::new(""));
        },
    }
}

async fn get_all_messages(url: &String) {
    match reqwest::get(&format!("{}/messages", url)).await {
        Ok(response) => {
            if response.status().is_success() {
                let messages: Vec<String> = response.json().await.expect("");
                println!("\n[=] All messages:");
                for msg in messages {
                    println!("\t-{}", msg);
                }
            } else {
                println!("[!] {}", response.status());
            }
        }
        Err(e) => println!("[!] {e}"),
    }
}

async fn options(url: &String) {
    // Client CLI
    loop {
        println!("[0] Exit");
        println!("[1] Download files");
        println!("[2] Read file");
        println!("[3] Send message");
        println!("[4] Get all messages");
        println!("[5] Check connection");
        
        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("[!] Cannot read choice");
        
        match choice.trim() {
            "0" => break,
            "1" => download_file(url).await.expect("[!] Err with download files"),
            "2" => read_file().await,
            "3" => _send_message(url).await.expect("[!] Err with sending message"),
            "4" => get_all_messages(url).await,
            "5" => check_connection(url).await.unwrap(),
            _ => println!("[!] Unknown command"),
        }
    }
}

pub async fn connect_as_guest() {
    let base_url: String = connect_to_network().await;
    options(&base_url).await;
}