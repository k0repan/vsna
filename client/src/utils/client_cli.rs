use std::io;
use crate::{
    config::Config,
    utils::file_handler,
    utils::client_connect::check_connection,
};

pub async fn client_cli(config: &Config) {
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