use std::io;

mod config;
mod utils;

use crate::config::Config;
use crate::utils::client_connect::client_connect;

async fn main_cli(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
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
            "0" => break Ok(()),
            "1" => client_connect(&config).await?,
            _ => println!("[!] Unknown command")
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = Config::new();
    println!("{config:?}");

    main_cli(&config).await?;
    Ok(())
}
