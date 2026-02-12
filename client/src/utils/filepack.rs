use std::time::UNIX_EPOCH;

use serde::{Serialize, Deserialize};
use tokio::fs;

#[cfg(unix)]
pub const PATH_DELIMETER: &str = "/";
#[cfg(not(unix))]
pub const PATH_DELIMETER: &str = "\\";

type BoxedErr = Box<dyn std::error::Error + Send + Sync>;

/// File size formatter (to B, KB, MB)
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

/// File message struct. Pack and unpack bytes of msg
/// TODO: encapsulate out, cuz server has the same
#[derive(Debug, Serialize, Deserialize)]
pub struct FilePacket {
    pub filename: String,
    size: u64,
    data: Vec<u8>,
    //last_modified: std::time::SystemTime,
}

impl FilePacket {
    /// Get FilePacket from file by its location
    pub async fn from_file(path: &String) -> Result<Self, BoxedErr> {
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
            //last_modified: metadata.modified().unwrap(),
        })
    }
    
    /// Parse Packet to Vec<u8> (bytes)
    pub fn to_bytes(&self) -> Result<Vec<u8>, BoxedErr> {
        let mut packet: Vec<u8> = Vec::new();
        
        let metadata_json: String = serde_json::to_string(self)?;
        let json_len: u32 = metadata_json.len() as u32;
        //let modified_time: &[u8; _] = &self.last_changes.duration_since(UNIX_EPOCH)?.as_secs().to_be_bytes();
        
        packet.extend_from_slice(&json_len.to_le_bytes());
        packet.extend_from_slice(metadata_json.as_bytes());
        //packet.extend_from_slice(modified_time);
        packet.extend_from_slice(&self.data);
        
        Ok(packet)
    }

    /// Parse Packet from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, BoxedErr> {
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
    
    /// Save file on PC in base_dir
    pub async fn save(&self, base_dir: &str) -> Result<String, BoxedErr> {
        let safe_name: &String = &self.filename;
        
        let path: String = format!("{}{}{}", base_dir, PATH_DELIMETER, safe_name);

        if fs::try_exists(&path).await? {
            return Err(format!("File '{}' already exists", safe_name).into());
        }
        
        if let Some(parent) = std::path::Path::new(&path).parent() {
            fs::create_dir_all(parent).await?;
        }
        
        fs::write(&path, &self.data).await?;
        
        Ok(get_file_size_str(self.size))
    }
}