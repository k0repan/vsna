use std::{fs, str::FromStr};
use serde_json;
use serde::{de, Deserialize};

/// JSON Config struct as env settings
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub client_path: String,
    pub addr: String,
    pub port: String,
    #[serde(deserialize_with = "boolean")]
    pub auto_sync: bool,
}

/// Convert json str to bool if it's "true"/"false"
fn boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where D: de::Deserializer<'de> {
    let s: String = String::deserialize(deserializer)?;
    bool::from_str(&s).map_err(de::Error::custom)
}

impl Config {
    /// Read __config__.json as env file
    pub fn new() -> Self {
        let res: String = fs::read_to_string("__config__.json")
        .expect("[!] Can't read json");

        serde_json::from_str(&res).unwrap()
    }

    /// Get WS addr to connect
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}