use std::io;
use std::path::Path;
use walkdir::{
    WalkDir,
    Error
};

// Read the path and files include, instead of ignored
pub fn read_path(){
    println!("[=] Enter the path (input ignored paths/files by !<path>)");
    println!("[=] Example: C:\\Users\\maksi\\AppData\\ !Roaming\\ !Localow\\ !*.log\n");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("[!] Failed to read line");

    let input: Vec<&str> = input.trim().split(" ").collect();
    let path: &str = input[0];
    let ignored: Vec<&str> = if input.len() > 1 { input[1..].to_vec() } else { Vec::new() };

    let mut path: String = String::from(path.trim());
    if !path.ends_with("\\") {
        path.push_str("\\");
    }

    let mut ignored_patterns: Vec<&str> = Vec::new();

    if ignored.len() > 0 {
        if ignored.iter().all(|&i|
            match i.chars().next() {
            Some(c) => c != '!',
            None => true,
        }) {
            println!("[!] Error: incorrrect ignored input");
            return;
        }
        // For now support only *file*, *.fmt patterns check
        for i in ignored {
            if i.ends_with("\\") {
                ignored_patterns.push(&i[1..i.len()-1]);
            } else if i.matches("*").count() == 1 || i.matches("*").count() == 0 {
                ignored_patterns.push(&i[1..]);
            } else if i.contains("*.") && i.matches("*").count() == 2{
                ignored_patterns.push(&i[2..i.len()-1]);
            } else {
                println!("[!] Unknown error");
                return;
            }
        }
    }
        
    let path: &str = &path;

    let path_obj: &Path = Path::new(path);
    if !path_obj.exists() {
        println!("[!] Error: Path '{}' does not exist", path);
        return;
    }

    if !path_obj.is_dir() {
        println!("[!] Error: '{}' is not a directory", path);
        return;
    }

    let _ = get_dirs_in_path(path, ignored_patterns);
}

// Print all dirs and files
fn get_dirs_in_path(path: &str, ignored_patterns: Vec<&str>) -> Result<(), Error>{
    for entry in WalkDir::new(path) {
        let entry: walkdir::DirEntry = entry?;
        let temp: String = entry.path().to_string_lossy().to_string();
        let temp_str: &str = temp.as_str();

        if ignored_patterns.iter().any(|&i| temp_str.contains(i)) {
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