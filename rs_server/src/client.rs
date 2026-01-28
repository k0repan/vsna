use std::{fs, io};
use axum::{Error};
use chrono::Utc;
use tokio::net::TcpStream;

use crate::host::get_addr;
use crate::structs::Message;
use crate::dir_handler::{
    read_path_as_client,
    get_dirs_in_path,
};
use crate::config::get_config;

async fn perform_handshake(addr: String, network_pwd: &str) -> Result<String, String> {
    let addr: String = format!("http://{}", get_addr());

    //TcpStream::connect(&addr).await.unwrap();

    match check_connection(&addr).await {
        Ok(_) => {
            println!("[=] Hadnshake performed");
            return Ok(addr);
        },
        Err(_) => {
            println!("[!] Err with handshake");
            return Err(addr);
        },
    };
}

async fn connect_to_network() -> String {
    // Connecting to VPN via name and pwd
    println!("[>] Input network name: ");
    let mut network_name: String = String::new();
    io::stdin()
    .read_line(&mut network_name)
    .expect("[!] Cannot read network name");
    let network_name: &str = network_name.trim();

    println!("[>] Input network password: ");
    let mut network_pwd: String = String::new();
    io::stdin()
    .read_line(&mut network_pwd)
    .expect("[!] Cannot read network password");
    let network_pwd: &str = network_pwd.trim();

    perform_handshake(network_name.to_string(), network_pwd).await.unwrap()
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

async fn read_file(with_input: bool, input: &mut String) {
    let input_file: String = match with_input {
        true => {
            println!("[=] You are in directory {}", &get_config().usr_path[2..]);
            println!("[>] Input file name: ");

            input.clear();
            io::stdin()
                .read_line(input)
                .expect("[!] Cannot read filename");

            input.trim().to_string()
        },
        false => input.to_string(),
    };

    let mut filename: String = match with_input{
        true => (&get_config().usr_path).to_string(),
        false => String::new(),
    };
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

async fn load_chat_history(url: &String) {
    let chat_files: String = get_dirs_in_path(
        get_config().chat_path.clone(),
        Vec::new()
    ).await.expect("[!] Failed read chat dir");
    
    println!("[=] Chat files:\n{}", chat_files);

    let mut file_choice: String = String::new();

    println!("[>] Which file upload?");
    io::stdin()
    .read_line(&mut file_choice)
    .expect("[!] Failed read choice");

    let filename: String = get_config().chat_path.to_owned() + &file_choice;
    let messages: Vec<String> = fs::read_to_string(filename.trim())
    .expect("[!] Err with reading file")
    .split("\n")
    .map(|s| s.to_string())
    .collect();

    for message in &messages{
        let from_and_text: Vec<String> = message.split(":").map(|s| s.to_string()).collect();

        let _text: Message = Message {
            from: from_and_text[0].trim().to_string(),
            text: from_and_text[1].trim().to_string(),
        };
        
        let client: reqwest::Client = reqwest::Client::new();
        match client.post(&format!("{}/send", url))
            .json(&_text) 
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
    }
}

async fn save_chat_history(url: &String) {
    let mut messages: Vec<String> = Vec::new();

    match reqwest::get(&format!("{}/messages", url)).await {
        Ok(response) => {
            if response.status().is_success() {
                messages = response.json().await.expect("");
            } else {
                println!("[!] {}", response.status());
            }
        }
        Err(e) => println!("[!] {e}"),
    }

    let mut timenow: String = Utc::now()
    .format("%Y%m%d_%H%M%S")
    .to_string();

    let dot: usize = if let Some(pos) = timenow.find(".") { // fuck me man
        pos
    } else {
        timenow.len()
    };

    timenow = timenow[..dot].to_string();

    let mut filename: String = format!("{}chat_{}.txt", get_config().chat_path, timenow);

    let content: String = messages.join("\n");

    fs::File::create(&filename).expect("[!] Failed saving chat history");
    fs::write(&filename, &content).expect("[!] Failed writing file");

    println!("[+] Chat history saved!");
    read_file(false, &mut filename).await;
}

async fn client_options(url: &String) {
    // Client CLI
    loop {
        println!("");
        println!("[0] Exit");
        println!("[1] Show files");
        println!("[2] Download files");
        println!("[3] Read file");
        println!("[4] Send message");
        println!("[5] Get all messages");
        println!("[6] Load chat history");
        println!("[7] Save chat history");
        println!("[8] Check connection");
        
        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("[!] Cannot read choice");
        
        match choice.trim() {
            "0" => break,
            "1" => show_files(url).await,
            "2" => download_file(url).await.expect("[!] Err with download files"),
            "3" => read_file(true, &mut String::new()).await,
            "4" => _send_message(url).await.expect("[!] Err with sending message"),
            "5" => get_all_messages(url).await,
            "6" => load_chat_history(url).await,
            "7" => save_chat_history(url).await,
            "8" => check_connection(url).await.unwrap(),
            _ => println!("[!] Unknown command"),
        }
    }
}

pub async fn connect_as_guest() {
    let base_url: String = connect_to_network().await;
    match check_connection(&base_url).await {
        Ok(_) => println!(),
        Err(_) => {
            return;
        },
    };
    client_options(&base_url).await;
}