use std::{env, sync::OnceLock};
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub usr_path: String,
    pub chat_path: String,
    pub addr: String,
    pub port: String,
    pub algo: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        
        Config {
            usr_path: Self::get_env_or_default("USR_PATH", "/usr/path/"),
            chat_path: Self::get_env_or_default("CHAT_PATH", "/usr/chat/"),
            addr: Self::get_env_or_default("ADDRESS", "0.0.0.0"),
            port: Self::get_env_or_default("PORT", "5555"),
            algo: Self::get_env_or_default("ENCRYPTED_ALGORITHM", "AES-256"),
        }
    }
    
    fn get_env_or_default(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }
}

static _CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    _CONFIG.get_or_init(Config::new)
}