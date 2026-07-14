use std::{
    io,
    path::Path,
};
use tokio_tungstenite::tungstenite::{Bytes, Message};
use tracing::error;
use walkdir::{WalkDir, Error};
use tokio::task;
use crate::{
    config::Config,
    utils::{filepack::FilePacket},
};

#[cfg(unix)]
pub const PATH_DELIMETER: &str = "/";
#[cfg(not(unix))]
pub const PATH_DELIMETER: &str = "\\";

/// CLI input stream reader
pub fn read_string() -> String {
    let mut read_str: String = String::new();
    io::stdin()
        .read_line(&mut read_str)
        .expect("[!] Err with readline");

    read_str.trim().to_string()
}

/// Receive and save file from bytes
pub async fn receive_file_from_server(path: &String, bytes: Bytes) {
    match FilePacket::from_bytes(&bytes) {
        Err(e) => eprintln!("[!] Err with unpack from bytes: {}", e),
        Ok(packet) => {
            println!("[=] Received file: {}", &packet.filename);
    
            match packet.save(path).await {
                Ok(size) => {
                    println!("[=] Downloaded: {}", size);
                },
                Err(e) => {
                    println!("{:?}", e);
                },
            }
        }
    }
}

/// Parse file from client_path to msg bytes
pub async fn get_bytes_of_file(path: &String, filename: &String) -> Option<Message> {
    let file_loc: String = format!("{}{}", &path, filename);
    println!("[=] Filename requested: {}", file_loc);

    if let Ok(packet) = FilePacket::from_file(&file_loc).await {
        if let Ok(bytes) = packet.to_bytes() {
            Some(Message::Binary(bytes.into()))
        } else {
            eprintln!("[!] Err with convert to bytes");
            None
        }
    } else {
        eprintln!("[!] Err with pack bytes");
        None
    }
}

/// Read the path and files include, instead of ignored
pub async fn get_struct_paths_files_with_ignored(config: &Config, input: &String) -> String {
    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let path: String = match input.get(0) {
        Some(s) => s.to_string(),
        None => String::from(""),
    };
    let ignored: Vec<&str> = if input.len() > 1 {
        input[1..].to_vec()
    } else {
        Vec::new()
    };

    let mut path: String = format!("{}{}", config.client_path, path.trim());

    if !path.ends_with(PATH_DELIMETER) {
        path.push_str(PATH_DELIMETER);
    }

    let mut ignored_patterns: Vec<String> = Vec::new();
    match get_ignored_patterns(&ignored).await {
        Ok(vec) => ignored_patterns.extend(vec),
        Err(_) => return "!".to_string(),
    };
        
    let path_obj: &Path = Path::new(&path);
    if !path_obj.exists() {
        error!("[!] Path {} does not exist", path);
        return "!".to_string();
    }

    if !path_obj.is_dir() {
        error!("[!] {} is not a directory", path);
        return "!".to_string();
    }

    str_struct_path(path, ignored_patterns).await.unwrap()
}

async fn get_ignored_patterns(ignored: &Vec<&str>) -> Result<Vec<String>, ()> {
    let mut ignored_patterns: Vec<String> = Vec::new();

    if ignored.len() > 0 {
        if ignored.iter().all(|&i|
            match i.chars().next() {
                Some(c) => c != '!',
                None => true,
            }) {
            error!("[!] Incorrrect ignored input");
            return Err(());
        }
        // For now support only *file*, *.fmt patterns check
        for i in ignored {
            if i.ends_with(PATH_DELIMETER) {
                ignored_patterns.push((&i[1..i.len()-1]).to_string());

            } else if i.matches("*").count() == 1 || i.matches("*").count() == 0 {
                ignored_patterns.push((&i[1..]).to_string());

            } else if i.contains("*.") && i.matches("*").count() == 2{
                ignored_patterns.push((&i[2..i.len()-1]).to_string());

            } else {
                error!("[!] Unknown error");
                return Err(());
            }
        }
    }

    Ok(ignored_patterns)
}

/// Print all dirs and files by layers
pub async fn str_struct_path (path: String, ignored_patterns: Vec<String>) -> Result<String, walkdir::Error> {    
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
            result.push_str(&format!(" {}\\ {}\\\n", indent, entry.file_name().to_string_lossy()));
        } else {
            result.push_str(&format!("{}| {}\n", indent, entry.file_name().to_string_lossy()));
        }
    }

    Ok(result)
}