mod config;
mod utils;

use crate::{
    config::Config,
    utils::{client_connect::client_connect, file_handler::read_string},
};

async fn main_cli(config: &Config) {
    loop {
        println!("");
        println!("[0] Exit");
        println!("[1] Connect");

        let choice: &str = &read_string();
        
        match choice {
            "0" => break,
            "1" => client_connect(&config).await,
            _ => println!("[!] Unknown command")
        }
    }
}

#[tokio::main]
async fn main() {
    if let Ok(config) = Config::new() {
        println!("{config:?}");
        main_cli(&config).await;
    } else {
        println!("[!] Err with make config")
    }
}
