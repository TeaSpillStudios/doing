use std::io::Write;
use std::net::TcpStream;

// TODO: Make an actual client.

fn main() {
    let mut stream = TcpStream::connect("localhost:2500").unwrap();

    stream.write(b"add_section|To do").unwrap();
}
