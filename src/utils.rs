use std::io::{Write, BufReader, BufRead};
use std::net::TcpStream;
use crate::Packet;

pub fn send_packet(packet: &Packet, stream: &mut TcpStream) {
    let json = serde_json::to_string(&packet).unwrap();

    let _ = stream.write_all(format!("{json}\n").as_bytes()).unwrap();
}

pub fn receive_packet(reader: &mut BufReader<&mut TcpStream>) -> Option<Packet> {
    let mut line = String::new();

    match reader.read_line(&mut line) {
        Ok(0) => None,
        Ok(_) => Some(serde_json::from_str(&line).unwrap()),
        Err(_) => None,
    }
}
