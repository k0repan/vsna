use std::path::Path;
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info};
use walkdir::{WalkDir, Error};
use tokio::task;
use crate::{
    config::Config,
    utils::filepack::FilePacket,
};

// Read the path and files include, instead of ignored
pub async fn _read_path(config: &Config, input: String) -> String {
    let input: Vec<&str> = input.trim().split(" ").collect();
    let path: String = input[0].to_string();
    let ignored: Vec<&str> = if input.len() > 1 {
        input[1..].to_vec()
    } else {
        Vec::new()
    };

    let mut path: String = format!("{}{}", config.server_path, path.trim());
    if !path.ends_with("\\") {
        path.push_str("\\");
    }

    let mut ignored_patterns: Vec<String> = Vec::new();

    if ignored.len() > 0 {
        if ignored.iter().all(|&i|
            match i.chars().next() {
                Some(c) => c != '!',
                None => true,
            }) {
            error!("[!] Incorrrect ignored input");
            return "!".to_string();
        }
        // For now support only *file*, *.fmt patterns check
        for i in ignored {
            if i.ends_with("\\") {
                ignored_patterns.push((&i[1..i.len()-1]).to_string());

            } else if i.matches("*").count() == 1 || i.matches("*").count() == 0 {
                ignored_patterns.push((&i[1..]).to_string());

            } else if i.contains("*.") && i.matches("*").count() == 2{
                ignored_patterns.push((&i[2..i.len()-1]).to_string());

            } else {
                error!("[!] Unknown error");
                return "!".to_string();
            }
        }
    }
        
    let path_obj: &Path = Path::new(&path);
    if !path_obj.exists() {
        error!("[!] Path {} does not exist", path);
        return "!".to_string();
    }

    if !path_obj.is_dir() {
        error!("[!] {} is not a directory", path);
        return "!".to_string();
    }

    get_dirs_in_path(path, ignored_patterns).await.unwrap()
}

// Print all dirs and files
pub async fn get_dirs_in_path(path: String, ignored_patterns: Vec<String>) -> Result<String, Error> {    
    let entries_result: Result<Vec<walkdir::DirEntry>, Error> = task::spawn_blocking(move || {
        WalkDir::new(path)
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
    }).await.unwrap();

    let entries: Vec<walkdir::DirEntry> = match entries_result {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("[!] Error walking directory: {}", e);
            return Err(e);
        }
    };

    let mut result: String = String::new();

    for entry in entries {
        let temp: String = entry.path().to_string_lossy().to_string();

        if ignored_patterns.len() > 0 {
            if ignored_patterns.iter().any(|pattern: &String| temp.contains(pattern)) {
                continue;
            }
        }

        let depth: usize = entry.depth();
        let indent: String = "  ".repeat(depth);
        
        if entry.file_type().is_dir() {
            result.push_str(&format!("{}📁 {}\\\n", indent, entry.file_name().to_string_lossy()));
        } else {
            result.push_str(&format!("{}📄 {}\n", indent, entry.file_name().to_string_lossy()));
        }
    }

    Ok(result)
}

pub async fn send_file_to_client(config: &Config, location: &String) -> Option<Message> {
    //TODO: Fragmentation, mayb RAR?
    let file_loc: String = format!("{}{}", &config.server_path, location);
    info!("Filename requested: {}", file_loc);

    let packet: FilePacket = FilePacket::from_file(&file_loc).await.expect("[!] Err with pack bytes");
    let bytes: Vec<u8> = packet.to_bytes().expect("[!] Err with convert to bytes");
    Some(Message::Binary(bytes.into()))
}