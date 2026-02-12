use std::{collections::HashSet, hash::Hash, path::{Path, PathBuf}};
use tokio_tungstenite::tungstenite::{Message, protocol::{CloseFrame, frame::coding::CloseCode}};
use tracing::{error, info};
use walkdir::{WalkDir, Error};
use tokio::task;
use crate::{
    config::Config,
    utils::{command_handler::ServerMsg, filepack::FilePacket},
};

#[cfg(unix)]
pub const PATH_DELIMETER: &str = "/";
#[cfg(not(unix))]
pub const PATH_DELIMETER: &str = "\\";

pub fn convert_msg_to_close(msg: String) -> Option<Message> {
    Some(Message::Close(Some(CloseFrame {
        code: CloseCode::Error,
        reason: msg.into(),
    })))
}

pub async fn get_server_path_str(config: &Config, input: &String) -> ServerMsg {
    let result_str: String = get_struct_paths_files_with_ignored(config, &input).await;
    if result_str.as_str() == "!" {
        vec![convert_msg_to_close(format!("Failed to resolve path: {}", input))]
    } else {
        vec![Some(Message::Text(result_str.into()))]
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

    let mut path: String = format!("{}{}", config.server_path, path.trim());

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

/// Get array of all files in path
pub async fn get_all_files_in_path(root: &String, ignored: &Vec<String>) -> Vec<PathBuf> {
    let root_str: String= root.clone();
    let ignored_clone: Vec<String> = ignored.clone();
    
    let files: Vec<PathBuf> = tokio::task::spawn_blocking(move || {
            let mut files: Vec<PathBuf> = Vec::new();
            for entry in WalkDir::new(&root_str) {
                let entry = entry.unwrap();
                let temp: String = entry.path().to_string_lossy().to_string();
                if ignored_clone.len() > 0 {
                    if ignored_clone.iter().any(|pattern: &String| temp.contains(pattern)) {
                        continue;
                    }
                }
                if entry.file_type().is_file() {
                    if let Ok(abs_path) = entry.path().canonicalize() {
                        if let Some(path_str) = abs_path.to_str() {
                            let trimmed: &str = &path_str[4..];
                            files.push(PathBuf::from(trimmed));
                        }
                    }
                }
            }
            return files;
        }
    ).await.unwrap();
    
    files
}

/// Fast duplicates erasement
pub fn clear_duplicates<T>(vec_to_clear: &Vec<T>) -> Vec<T>
where T: Clone + Hash + Eq,
{
    let mut seen: HashSet<&T> = HashSet::new();
    let mut result: Vec<T> = Vec::new();
    
    for item in vec_to_clear {
        if seen.insert(item) {
            result.push(item.clone());
        }
    }

    result
}

/// Parse file data to bytes and send to client
pub async fn send_file_to_client(config: &Config, location: &String) -> ServerMsg {
    //TODO: Fragmentation, mayb RAR?
    let mut vec_msg: Vec<Option<Message>> = Vec::new();
    let root: String = format!("{}{}", &config.server_path, location);
    let vec_files: Vec<PathBuf> = clear_duplicates(&get_all_files_in_path(&root, &Vec::new()).await);

    for loc in vec_files {
        info!("Filename requested: {}", loc.to_string_lossy());
        let packet: FilePacket = FilePacket::from_file(&loc.to_string_lossy().to_string()).await.expect("[!] Err with pack bytes");
        
        if packet.check_size() {
            match packet.to_bytes() {
                Ok(bytes) => {
                    let msg: Option<Message> = Some(Message::Binary(bytes.into()));
                    vec_msg.push(msg);
                },
                Err(e) => {
                    error!("{}", e);
                    vec_msg.push(convert_msg_to_close(format!("Err with Filepacket.to_bytes: {}", e)));
                },
            }
        } else {
            vec_msg.push(None);
        }
    }
    vec_msg.push(Some(Message::Binary("cargo is ass".into())));
    vec_msg
}

pub async fn save_file_server() -> Vec<Option<Message>> {
    vec![Some(Message::Pong("".into()))]
}

/// Save file by sended bytes from client
pub async fn save_file_bytes_server(config: &Config, bytes: &[u8]) -> Message {
    match FilePacket::from_bytes(&bytes) {
        Ok(packet) => {
            info!("Received file: {}", &packet.filename);
            
            match packet.save(&config.server_path).await {
                Ok(_) => (),
                Err(e) => {
                    error!("Err with saving file: {}", e);
                    return convert_msg_to_close(e.to_string()).unwrap();
                },
            };
            Message::Text(packet.get_size().to_string().into())
        },
        Err(e) => {
            error!("Err with from_bytes: {}", e);
            return convert_msg_to_close(e.to_string()).unwrap();
        },
    }
}