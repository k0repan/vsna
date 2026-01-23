use axum::{
    Error,
    Router,
    extract::State,
    response::Json,
    routing::{get, post},
};
use serde::{
    Deserialize,
    Serialize,
};
use std::{fs, io, sync::Arc};
use tokio::{
    signal,
    sync::Mutex,
};

use crate::config::get_config;
use crate::dir_handler::read_path_as_host;
use crate::client::check_connection;

#[derive(Clone)]
struct AppState {
    shared_data: Arc<Mutex<Vec<String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub from: String,
}

pub fn get_addr() -> String {
    format!("{}:{}", get_config().addr, get_config().port)
}

async fn hello_world() -> &'static str {
    "Hello from Axum server!"
}

async fn send_message(
    State(state): State<AppState>,
    Json(message): Json<Message>
) -> Json<Message> {
    let mut data: tokio::sync::MutexGuard<'_, Vec<String>> = state.shared_data.lock().await;
    data.push(format!("{}: {}", message.from, message.text));
    
    Json(Message {
        text: format!("[+] Message's received. Total messages: {}", data.len()),
        from: "server".to_string(),
    })
}

async fn get_messages(State(state): State<AppState>) -> Json<Vec<String>> {
    let data: tokio::sync::MutexGuard<'_, Vec<String>> = state.shared_data.lock().await;
    Json(data.clone())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("[!] Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("[!] Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn check_env_paths() -> Result<(), io::Error>{
    fs::create_dir_all(&get_config().usr_path)?;
    fs::create_dir_all(&get_config().chat_path)?;

    Ok(())
}

async fn start_server() -> Result<(), axum::Error> {
    let addr: String = get_addr();

    match check_connection(&format!("http://{}", addr)).await {
        Ok(_) => {
            eprintln!("[!] Server is already running");
            return Ok(());
        },
        Err(_) => println!("[=] Port is OK"),
    };

    check_env_paths().await.expect("[!] Env paths not found.");

    let state: AppState = AppState {
        shared_data: Arc::new(Mutex::new(Vec::new())),
    };
    
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/send", post(send_message))
        .route("/messages", get(get_messages))
        .route("/files", get(read_path_as_host))
        .with_state(state);

    println!("[=] Server is on http://{}", addr); 
    
    let _ = tokio::spawn(async move {        
        axum::serve(
            tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
        ).with_graceful_shutdown(async {
            shutdown_signal().await;
            println!("[=] Server is shutdowned successfully.");
        }).await.unwrap();
    });
    
    Ok(())
}

pub async fn stop_server() -> Result<(), Error> {
    match check_connection(&format!("http://{}", get_addr())).await {
        Ok(_) => {
            println!("[=] Stopping server...");
            println!("[=] Press Ctrl+C to shutdown");
        },
        Err(_) => {
            eprintln!("[!] Server is not running");
        },
    };

    Ok(())
}

async fn run_vpn() -> Result<(), String> {
    // Smth happened which can run VPN with Host and Client
    
    Ok(())
}

pub async fn run_as_host() -> Result<(), String> {
    start_server().await.expect("[!] Err with starting server");
    run_vpn().await.expect("[!] Err with run VPN");
    read_path_as_host().await;
    
    Ok(())
}