use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = {
        let _ = dotenv();
        
        Config {
            usr_path: env::var("USR_PATH")
                .unwrap_or_else(|_| "/usr".to_string()),
            
            addr: env::var("ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            
            port: env::var("PORT")
                .unwrap_or_else(|_| "5555".to_string()),
            
            algo: env::var("ENCRYPTED_ALGORITHM")
                .unwrap_or_else(|_| "AES-256".to_string())
        }
    };
}

pub struct Config {
    pub usr_path: String,
    pub addr: String,
    pub port: String,
    pub algo: String,
}