use std::{
    io::{stdin, stdout, Read, Write},
    net::{Shutdown, TcpStream},
};
use tracing::{error, info};

fn execute_command(command: &str) -> String {
    let mut stream = match TcpStream::connect("localhost:2500") {
        Ok(v) => v,
        Err(e) => {
            error!("Connection refused.\n                 ╰ {e}");
            return String::from("Failure.");
        }
    };

    stream.write(command.as_bytes()).unwrap();

    let mut buffer = [0; 8096];
    let bytes_read = stream.read(&mut buffer).unwrap();

    if let Err(e) = stream.shutdown(Shutdown::Both) {
        error!("Failed to shutdown stream: {e}");
    }

    String::from_utf8_lossy(&buffer[..bytes_read])
        .lines()
        .map(|v| format!("    {v}\n"))
        .collect::<String>()
}

fn main() {
    tracing_subscriber::fmt().without_time().init();

    loop {
        let mut buf = String::new();
        print!("Command: ");

        stdout().flush().unwrap();
        stdin().read_line(&mut buf).unwrap();

        let buf = buf.trim();

        if buf == String::from("break") {
            break;
        }

        let response = execute_command(&buf);
        info!("Response: \n{}", response);
    }

    // let response = execute_command("list|test");
    // info!("Response: \n{}", response);

    // thread::sleep(Duration::from_millis(100));

    // let response = execute_command("list|test");
    // info!("Response: \n{}", response);
}
