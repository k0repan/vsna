use std::io;

mod dir_handler;
mod config;
mod host;
mod client;

/*
Output signs:
\ - dir | path,
= - info,
+ - write,
! - err,
1..9 - choice
*/

async fn run_cli() {
    // Main CLI
    loop {
        println!("[0] Exit");
        println!("[1] Run as host");
        println!("[2] Connect as client");
        
        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("[!] Cannot read choice");
        
        match choice.trim() {
            "0" => break,
            "1" => host::run_as_host().await.expect(""),
            "2" => client::connect_as_guest().await,
            _ => println!("[!] Unknown command"),
        }
    }
}

#[tokio::main]
async fn main() {
    run_cli().await;
}