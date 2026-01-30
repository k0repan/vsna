use std::{io::{self, Write}, net::{SocketAddr, TcpStream}, str::FromStr, time::Duration};
use crate::{config::Config, utils::file_handler};

async fn loading_spinner(duration_secs: u64) {
    let frames = vec!["-", "\\", "|", "/"];
    let end_time = Duration::from_secs(duration_secs);
    let start = std::time::Instant::now();
    
    while start.elapsed() < end_time {
        for frame in &frames {
            print!("\r{} Loading...", frame);
            io::stdout().flush().unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
    println!("");
}

pub async fn client_connect(config: &Config) {
    let mut ip: String = String::new();
    println!("[>] Input IP addr:");
    io::stdin()
        .read_line(&mut ip)
        .expect("[!] Err with reading IP");

    let mut port: String = String::new();
    println!("[>] Input Port:");
    io::stdin()
        .read_line(&mut port)
        .expect("[!] Err with reading Port");
    let ip: &str = ip.trim();
    let port: &str = port.trim();

    let addr: String = if ip.is_empty() || port.is_empty() {
        config.get_addr()
    } else {
        format!("{}:{}", ip, port)
    };

    tokio::spawn(loading_spinner(4));

    match TcpStream::connect_timeout(&SocketAddr::from_str(&addr.as_str()).unwrap(), Duration::from_secs(5)) {
        Ok(_) => println!("[=] Hadnshake performed. Addr: {}", &addr),
        Err(_) => {
            println!("[!] Err with handshake. Addr: {}", &addr);
            return;
        }
    }

    client_cli(config).await;
}

async fn client_cli(config: &Config) {
    let url: String = config.get_addr();

    loop {
        println!("");
        println!("[0] Exit");
        println!("[1] Show path");
        println!("[2] Download files");
        println!("[3] Check connection");

        if !check_connection(&url).await {
            break;
        }

        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("[!] Err with read choice");

        let choice: &str = choice.trim();
        
        match choice {
            "0" => break,
            "1" => show_path(&url).await,
            "2" => download_files(&url, &config.client_path).await,
            "3" => {
                if check_connection(&url).await {
                    println!("[=] Connection is successfull");
                } else {
                    println!("[!] Err with connection");
                }
            },
            _ => println!("[!] Unknown command")
        }
    }
}


async fn show_path(url: &String) {
    if !check_connection(&url).await {
        return;
    }

    // TODO: пока такой костыль, потом исправить по-другому надо
    match reqwest::get(&format!("{}/files", url)).await {
        Ok(response) => {
            let files: String = response.text().await.unwrap();
            println!("\n[=] All files:\n{}", files);
        }
        Err(e) => println!("[!] {e}"),
    }
}

async fn download_files(url: &String, client_path: &String) {
    if !check_connection(&url).await {
        return;
    }

    if file_handler::_read_path(&client_path).await == "!" {
        return;
    } else {
        // not err
    }
}

pub async fn check_connection(url: &String) -> bool {
    TcpStream::connect(url).is_ok()
}