use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("0.0.0.0:2345").unwrap();
    println!("Connected to server!");

    stream.write_all(b"Privet from client!");

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    let response = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Server replied: {}", response);
}
