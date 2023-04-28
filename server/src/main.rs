use log::info;
use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

mod task_handler;
use task_handler::*;

const PORT: &str = "2500";

fn main() {
    pretty_env_logger::init();

    let mut task_handler = TaskHandler::default();

    info!("Listening on port {PORT}");

    let listener = match TcpListener::bind(&format!("0.0.0.0:{PORT}")) {
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

    // TODO: Add error handling.
    let lines = BufReader::new(stream).lines();

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

    handle_command_args(task_handler, args)
}

fn handle_command_args(task_handler: &mut TaskHandler, args: Vec<&str>) {
    let command = args[0];

    if args.len() > 2 {
        println!("3 params detected");
        println!("{}", args.len());
    }
}
