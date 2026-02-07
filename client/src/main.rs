mod config;
mod utils;

use std::io;
use crate::{
    config::Config,
    utils::client_connect::client_connect,
};

async fn main_cli(config: &Config) {
    loop {
        println!("");
        println!("[0] Exit");
        println!("[1] Connect");

        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("[!] Err read choice");

        let choice: &str = choice.trim();
        
        match choice {
            "0" => break,
            "1" => client_connect(&config).await,
            _ => println!("[!] Unknown command")
        }
    }
}

#[tokio::main]
async fn main() {
    let config: Config = Config::new();
    println!("{config:?}");

    main_cli(&config).await;
}
