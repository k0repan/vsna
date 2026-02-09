use serde::{Serialize, Deserialize};
use tokio::fs;

#[cfg(unix)]
pub const PATH_DELIMETER: &str = "/";
#[cfg(not(unix))]
pub const PATH_DELIMETER: &str = "\\";

fn get_file_size_str(size: u64) -> String {
    const UNITS: [&str; 3] = ["B", "KB", "MB"];
    
    if size == 0 {
        return "0 B".to_string();
    }
    
    let base: f64 = 1024.0;
    let exponent: usize = ((size as f64).log(base).floor() as usize).min(UNITS.len() - 1);
    
    let size: f64 = size as f64 / base.powi(exponent as i32);
    
    if exponent == 0 {
        format!("{} {}", size, UNITS[exponent])
    } else {
        format!("{:.2} {}", size, UNITS[exponent])
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePacket {
    pub filename: String,
    size: u64,
    data: Vec<u8>,
}

impl FilePacket {
    pub async fn from_file(path: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let data: Vec<u8> = fs::read(path).await?;
        let metadata: std::fs::Metadata = fs::metadata(path).await?;
        
        let filename: String = std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file.bin")
            .to_string();
        
        Ok(Self {
            filename,
            size: metadata.len(),
            data,
        })
    }
    
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let metadata_json: String = serde_json::to_string(self)?;
        
        let mut packet: Vec<u8> = Vec::new();
        
        let json_len: u32 = metadata_json.len() as u32;
        packet.extend_from_slice(&json_len.to_le_bytes());
        packet.extend_from_slice(metadata_json.as_bytes());
        packet.extend_from_slice(&self.data);
        
        Ok(packet)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if data.len() < 4 {
            return Err("Not enough data".into());
        }
        
        let json_len: usize = u32::from_le_bytes(data[0..4].try_into()?) as usize;
        
        if 4 + json_len > data.len() {
            return Err("Incorrect JSON size".into());
        }
        
        let json_data: &[u8] = &data[4..4 + json_len];
        let mut packet: FilePacket = serde_json::from_slice(json_data)?;
        
        let file_data_start: usize = 4 + json_len;
        packet.data = data[file_data_start..].to_vec();
        
        if packet.data.len() as u64 != packet.size {
            return Err(format!("The sizes don't match: waited for {}, received {}", 
                packet.size, packet.data.len()).into());
        }
        
        Ok(packet)
    }
    
    pub async fn save(&self, base_dir: &str) -> Result<String, Box<dyn std::error::Error>> {
        let safe_name: &String = &self.filename;
        
        let path: String = format!("{}{}{}", base_dir, PATH_DELIMETER, safe_name);

        if fs::try_exists(&path).await? {
            return Err(format!("File '{}' already exists", safe_name).into());
        }
        
        if let Some(parent) = std::path::Path::new(&path).parent() {
            fs::create_dir_all(parent).await?;
        }

        println!("[=] Downloaded: {}", get_file_size_str(self.size));
        
        fs::write(&path, &self.data).await?;
        
        Ok(path)
    }
}