use std::io;
use axum::{
    Error
};

use crate::config::Config;

pub async fn client_connect(config: Config) {
    // TODO: do real connect
    let addr: String = config.clone().addr;

    //TcpStream::connect(&addr).await.unwrap();

    match check_connection(&addr).await {
        Ok(_) => println!("[=] Hadnshake performed"),
        Err(_) => {
            println!("[!] Err with handshake");
            return;
        }
    };

    client_cli(config).await;
}

async fn client_cli(config: Config) {
    let url: String = config.addr;

    loop {
        println!("[0] Exit");
        println!("[1] Show path");
        println!("[2] Download files");
        println!("[3] Check connection");

        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("[!] Err read choice");

        let choice: &str = choice.as_str().trim();
        
        match choice {
            "0" => break,
            "1" => show_path(&url).await,
            "2" => download_files(),
            "3" => check_connection(&url).await.unwrap(),
            _ => println!("[!] Unknown command")
        }
    }
}


async fn show_path(url: &String) {
    // TODO: пока такой костыль, потом исправить по-другому надо
    match reqwest::get(&format!("{}/files", url)).await {
        Ok(response) => {
            if response.status().is_success() {
                let files: String = response.text().await.unwrap();
                println!("\n[=] All files:\n{}", files);
            } else {
                println!("[!] {}", response.status());
            }
        }
        Err(e) => println!("[!] {e}"),
    }
}

fn download_files() {
    // TODO: make smth
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