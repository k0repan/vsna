use std::{fs, io};
use axum::Error;

use crate::host::{
    Message,
    get_addr,
};
use crate::dir_handler::read_path_as_client;
use crate::config::get_config;

async fn connect_to_network() -> String {
    // Connecting to VPN via name and pwd
    println!("[>] Input IP of the server (localhost, 192.0.12.3): ");
    let mut server_ip: String = String::new();
    io::stdin().read_line(&mut server_ip).expect("[!] Cannot read IP");
    let server_ip: &str = server_ip.trim();

    if server_ip.len() == 0 {
        return format!("http://{}", get_addr());
    }
    
    format!("http://{}:{}", server_ip, get_config().port) // return url
}

async fn show_files(url: &String) {
    match reqwest::get(&format!("{}/files", url)).await {
        Ok(response) => {
            if response.status().is_success() {
                let files: String = response.text().await.expect("");
                println!("\n[=] All files:\n{}", files);
            } else {
                println!("[!] {}", response.status());
            }
        }
        Err(e) => println!("[!] {e}"),
    }
}

async fn download_file(url: &String) -> Result<(), String> {
    read_path_as_client().await;
    // input file names & copy files to client device
    Ok(())
}

async fn read_file() {
    println!("[=] You are in directory {}", &get_config().usr_path[2..]);
    println!("[>] Input file name: ");
    let mut input_file: String = String::new();
    io::stdin()
        .read_line(&mut input_file)
        .expect("[!] Cannot read filename");

    let input_file: &str = input_file.trim();

    let mut filename: String = (&get_config().usr_path).to_string();
    filename.push_str(&input_file);

    let filesize: u64 = fs::metadata(&filename).unwrap().len();

    if filesize > 1024 {
        println!("[!] Filesize is too big: {}", filesize);
        return;
    }

    match fs::read_to_string(&filename) {
        Ok(content) => {
            println!("{} {}B:\n{}", filename, filesize, content);
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
        println!("");
        println!("[0] Exit");
        println!("[1] Show files");
        println!("[2] Download files");
        println!("[3] Read file");
        println!("[4] Send message");
        println!("[5] Get all messages");
        println!("[6] Check connection");
        
        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("[!] Cannot read choice");
        
        match choice.trim() {
            "0" => break,
            "1" => show_files(url).await,
            "2" => download_file(url).await.expect("[!] Err with download files"),
            "3" => read_file().await,
            "4" => _send_message(url).await.expect("[!] Err with sending message"),
            "5" => get_all_messages(url).await,
            "6" => check_connection(url).await.unwrap(),
            _ => println!("[!] Unknown command"),
        }
    }
}

pub async fn connect_as_guest() {
    let base_url: String = connect_to_network().await;
    match check_connection(&base_url).await {
        Ok(_) => println!(),
        Err(e) => eprintln!("[!] {}", e),
    };
    options(&base_url).await;
}