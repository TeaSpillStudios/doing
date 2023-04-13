mod task_handler;
use task_handler::*;

use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

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

        handle_request(stream, &mut task_handler);
    }
}

fn handle_request(mut stream: TcpStream, task_handler: &mut TaskHandler) {
    log::info!("Recieved a connection.");

    let lines = BufReader::new(stream).lines();

    let command = String::new();

    let last_tine = match lines.last() {
        Some(v) => v,
        None => {
            log::error!("No command passed.");
            return;
        }
    };

    let last_line = match &last_tine {
        Ok(v) => v,
        Err(e) => {
            log::error!("{e}");
            return;
        }
    };

    let args: Vec<&str> = last_line.split("|").collect();
    let command = args[0];
    let args = args[1..].iter().map(|v| *v).collect::<Vec<&str>>();

    dbg!(command);
    dbg!(args);
}
