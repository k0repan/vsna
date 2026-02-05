use bstr::ByteSlice;

#[derive(Debug)]
pub struct UserFileMessage {
  pub username: String,
  pub file_name: String,
  pub file_type: String,
  pub file_bytes: Vec<u8>,
}

impl UserFileMessage {
  pub fn from(data: Vec<u8>) -> Option<Self> {
    // Паттерн разделителя, который отделяет каждый фрагмент 
    // информации от другого. Например:
    // username<pattern>filename<pattern>filetype<pattern>filebytes
    let pattern: [u8; 12] = [226, 128, 147, 226, 128, 147, 226, 128, 147, 226, 128, 147]; 
    
    // Делит массив по паттерну
    let result: Vec<Vec<u8>> = data.split_str(&pattern).map(|x| x.to_vec()).filter(|x| x.len() > 0).collect();
    
    if result.len() == 4 {
      let username: String = String::from_utf8_lossy(&result[0]).to_string();
      let file_name: String = String::from_utf8_lossy(&result[1]).to_string();
      let file_type: String = String::from_utf8_lossy(&result[2]).to_string();
      
      let file_bytes: Vec<u8> = result[3].clone();
      
      Some(UserFileMessage { username, file_name, file_type, file_bytes })
    } else {
      None
    }
  }
}