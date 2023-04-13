mod task_handler;
use task_handler::*;

use std::net::{TcpListener, TcpStream};

fn main() {
    pretty_env_logger::init();

    let mut task_handler = TaskHandler::default();

    let listener = match TcpListener::bind("0.0.0.0:2500") {
        Ok(v) => v,
        Err(e) => {
            log::error!("{e}");
            return;
        }
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(e) => {
                log::error!("{e}");
                return;
            }
        };

        handle_request(stream);
    }
}

fn handle_request(mut stream: TcpStream) {
    log::info!("Recieved a connection.");
}
