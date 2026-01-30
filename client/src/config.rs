use std::fs;
use serde_json;
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub client_path: String,
    pub addr: String,
    pub auto_sync: u16,
}

impl Config {
    pub fn new() -> Self {
        let res = fs::read_to_string("__config__.json")
        .expect("[!] Can't read json");

        serde_json::from_str(&res).unwrap()
    }
}