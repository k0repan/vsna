use std::io;

use axum::{
    routing::get,
    Router,
};

mod dir_handler;

const ADDR: &str = "0.0.0.0";
const PORT: &str = "3000";


fn get_full_addr() -> String {
    format!("{}:{}", ADDR, PORT)
}


fn std_addr() -> &'static str {
    "Hello, world!"
}

async fn start_server() {
    // build our application with a single route
    let app: Router = Router::new().route("/", get({ std_addr(); }));

    // run our app with hyper, listening globally on port 3000
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(get_full_addr()).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


#[tokio::main]
async fn main() {
    loop {
            println!("
[0] Exit
[1] Run as host
[2] Connect as guest");

        let mut choose: String = String::new();

        io::stdin()
            .read_line(&mut choose)
            .expect("[!] Failed to read line");

        let choose: u8 = choose
                            .trim()
                            .parse()
                            .expect("[!] Please type a number!");

        match choose {
            0 => break,
            1 => dir_handler::read_path(),
            2 => println!("[!] Function in dev..."),
            _ => println!("[!] Unknown command"),
        }
    }
}