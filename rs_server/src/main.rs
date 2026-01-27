use std::io;

mod dir_handler;
mod config;
mod host;
mod client;
mod structs;

/*
Output signs []:
\ - dir | path,
= - info,
+ - write,
> - input,
! - err,
~ - dbg,
1..9 - choice
*/

async fn run_cli() {
    // Main CLI
    loop {
        //println!("[~] ENV: {:?}", config::get_config());
        println!("");
        println!("[0] Exit");
        println!("[1] Run as host");
        println!("[2] Connect as client");
        println!("[3] Stop server");
        
        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("[!] Cannot read choice");
        
        match choice.trim() {
            "0" => break,
            "1" => host::run_as_host().await.expect(""),
            "2" => client::connect_as_guest().await,
            "3" => host::stop_server().await.expect(""),
            _ => println!("[!] Unknown command"),
        }
    }
}

#[tokio::main]
async fn main() {
    run_cli().await;
}