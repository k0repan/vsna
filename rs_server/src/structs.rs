use serde::{
    Deserialize,
    Serialize,
};
use std::sync::Arc;
use std::net::IpAddr;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub shared_data: Arc<Mutex<Vec<String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub from: String,
}

pub struct Network {
    name: String,
    password_hash: String,
    clients: Vec<Client>,
}

pub struct Client {
    name: String,
    public_key: String,
    ip_address: Option<IpAddr>
}