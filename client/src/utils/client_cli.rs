use crate::{
    config::Config,
    utils::{
        file_handler::read_string,
        ws::WebSocketClient,
        command_handler::ClientCommandHandler,
    },
};

/// Second CLI layer, if it successfully connected to WS
pub async fn client_cli(config: &Config, ws_stream: WebSocketClient) {
    let client_handler: ClientCommandHandler = ClientCommandHandler::new(config, ws_stream);
    loop {
        println!("");
        println!("[0] Exit");
        println!("[1] Show server path");
        println!("[2] Download files");
        println!("[3] Send files");
        println!("[4] Check connection");

        let choice: &str = &read_string();
        
        match choice {
            "0" => break,
            "1" => client_handler.show_path_request().await,
            "2" => client_handler.download_files_request().await,
            "3" => client_handler.send_files_request().await,
            "4" => {
                match client_handler.check_connection().await {
                    Ok(s) => {
                        println!("{}", s);
                        continue;
                    },
                    Err(e) => {
                        println!("{}", e);
                        break;
                    }
                }
            },
            _ => println!("[!] Unknown command"),
        }

        if let Err(e) = client_handler.check_connection().await {
            println!("{}", e);
            break;
        }
    }
}