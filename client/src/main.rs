use std::io;

mod config;
mod utils;

use crate::config::Config;
use crate::utils::client_cli::client_connect;

async fn main_cli(config: Config) {
    loop{
        println!("[0] Exit");
        println!("[1] Connect");

        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("[!] Err read choice");

        let choice: &str = choice.as_str().trim();
        
        match choice {
            "0" => break,
            "1" => client_connect(config.clone()).await,
            _ => println!("[!] Unknown command")
        }
    }
}

#[tokio::main]
async fn main() {
    let config: Config = Config::new();
    println!("{config:?}");

    main_cli(config).await;
}
