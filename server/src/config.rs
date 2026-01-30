use std::fs;
use serde_json;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_path: String,
    pub max_size: u16,
    pub max_clients: u16,
}

impl Config {
    pub fn new() -> Self {
        let res = fs::read_to_string("__config__.json")
        .expect("[!] Can't read json");

        serde_json::from_str(&res).unwrap()
    }
}