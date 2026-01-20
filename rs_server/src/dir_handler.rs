use std::{
    io,
    path::Path,
    fs
};
use walkdir::{
    WalkDir,
    Error
};
use tokio::task;
use axum::response::Json;


pub async fn list_files() -> Json<Vec<String>> {
    let mut files: Vec<String> = Vec::new();
    
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                if let Ok(name) = entry.file_name().into_string() {
                    files.push(name);
                }
            }
        }
        Err(_) => eprintln!("[!] Error reading paths"),
    }
    
    Json(files)
}

pub async fn read_path() {
    println!("[\\] Current path:");
    
    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                if let Ok(name) = entry.file_name().into_string() {
                    println!("\t-{}", name);
                }
            }
        }
        Err(e) => eprintln!("[!] {e}"),
    }
}

// Read the path and files include, instead of ignored
pub async fn _read_path() {
    println!("[=] Enter the path (input ignored paths/files by !path\\ | !*.fmt | !file)");
    println!("[=] Example: C:\\Users\\maksi\\AppData\\ !Roaming\\ !*.log\n");

    let input: String = tokio::task::spawn_blocking(|| {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("[!] Failed to read line");
        input
    }).await.unwrap();

    let input: Vec<&str> = input.trim().split(" ").collect();
    let path: String = input[0].to_string();
    let ignored: Vec<&str> = if input.len() > 1 {
        input[1..].to_vec()
    } else {
        Vec::new()
    };

    let mut path: String = String::from(path.trim());
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
            println!("[!] Incorrrect ignored input");
            return;
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
                println!("[!] Unknown error");
                return;
            }
        }
    }
        
    let path_obj: &Path = Path::new(&path);
    if !path_obj.exists() {
        println!("[!] Path '{}' does not exist", path);
        return;
    }

    if !path_obj.is_dir() {
        println!("[!] '{}' is not a directory", path);
        return;
    }

    get_dirs_in_path(path, ignored_patterns).await.unwrap();
}

// Print all dirs and files
async fn get_dirs_in_path(path: String, ignored_patterns: Vec<String>) -> Result<(), Error> {    
    let entries_result: Result<Vec<walkdir::DirEntry>, Error> = task::spawn_blocking(move || {
        WalkDir::new(&path)
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

    for entry in entries {
        let temp: String = entry.path().to_string_lossy().to_string();

        if ignored_patterns.iter().any(|pattern| temp.contains(pattern)) {
            continue;
        }

        let depth: usize = entry.depth();
        let indent: String = "  ".repeat(depth);
        
        if entry.file_type().is_dir() {
            println!("{}📁 {}\\", indent, entry.file_name().to_string_lossy());
        } else {
            println!("{}📄 {}", indent, entry.file_name().to_string_lossy());
        }
    }
    
    Ok(())
}