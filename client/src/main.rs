use log::{error, info};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

// TODO: Make an actual client.

fn main() {
    pretty_env_logger::init();

    let mut stream = match TcpStream::connect("localhost:2500") {
        Ok(v) => v,
        Err(e) => {
            error!("Connection refused.\n                 ╰ {e}");
            return;
        }
    };

    info!("Writing");

    // stream.write(b"list|To do|Hi\n").unwrap();
    stream.write(b"list|test").unwrap();

    info!("Reading\n");

    let mut buffer = [0; 8096];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let response = String::from_utf8_lossy(&buffer[..bytes_read])
        .lines()
        .map(|v| format!("    {v}\n"))
        .collect::<String>();

    info!("Response: \n{}", response);
}
