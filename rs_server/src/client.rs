use std::io;
use crate::host::Message;

async fn connect_to_network() -> String {
    // Connected to VPN via name and pwd
    println!("[>] Input IP of the server (localhost, 192.0.12.3): ");
    let mut server_ip: String = String::new();
    io::stdin().read_line(&mut server_ip).expect("[!] Cannot read IP");
    let server_ip: &str = server_ip.trim();
    
    format!("http://{}:5555", server_ip) // return url
}

async fn download_file() -> Result<(), String> {
    // dir_handler::read_path().await?;
    
    Ok(())
}

async fn _send_message() -> Result<(), String> {
    // smth
    
    Ok(())
}

async fn options() -> Result<(), String>{
    // Client CLI
    loop {
        println!("[0] Exit");
        println!("[1] Download file");
        println!("[2] Send message");
        
        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("[!] Cannot read choice");
        
        match choice.trim() {
            "0" => (),
            "1" => download_file().await?,
            "2" => _send_message().await?,
            _ => println!("[!] Unknown command"),
        }
    }
}

pub async fn connect_as_guest() {
    let base_url: String = connect_to_network().await;

    // options().await;
    
    loop {
        println!("[0] Exit");
        println!("[1] Check connection");
        println!("[2] Send message");
        println!("[3] Get all messages");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("[!] Cannot read choose");
        
        match choice.trim() {
            "1" => {
                match reqwest::get(&base_url).await {
                    Ok(response) => {
                        if response.status().is_success() {
                            let text: String = response.text().await.unwrap_or_default();
                            println!("[+] Connnect successful! Response: {}", text);
                        } else {
                            println!("[!] {}", response.status());
                        }
                    }
                    Err(e) => println!("[!] {e}"),
                }
            }
            "2" => {
                println!("[>] Input name: ");
                let mut from: String = String::new();
                io::stdin().read_line(&mut from).expect("[!] Cannot read name");
                
                println!("[>] Input message: ");
                let mut text: String = String::new();
                io::stdin().read_line(&mut text).expect("[!] Cannot read message");
                
                let message: Message = Message {
                    text: text.trim().to_string(),
                    from: from.trim().to_string(),
                };
                
                let client: reqwest::Client = reqwest::Client::new();
                match client.post(&format!("{}/send", base_url))
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
            }
            "3" => {
                match reqwest::get(&format!("{}/messages", base_url)).await {
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
            "0" => break,
            _ => println!("[!] Unknown command"),
        }
    }
}