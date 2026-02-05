use tokio::fs::File;
use tokio_tungstenite::tungstenite::Bytes;

pub async fn receive_file_from_server(path: &String, bytes: Bytes) {
    // TODO: unpack bytes to file
    tokio::fs::write(path, bytes).await.expect("[!] Err with writing file");
}

pub async fn get_bytes_of_file(path: &String, filename: &String) -> Vec<u8> {
    let file: File = File::open(format!("{}{}", path, filename))
        .await
        .expect(format!("[!] Err with open file: {}", filename).as_str());
    //somehow convert file by filename into the bytes
    "".into()
}