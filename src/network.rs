use std::io::{BufRead, Write};

use crate::protocol::Packet;

pub fn send_packet<W: Write>(packet: &Packet, writer: &mut W) {
    let json = serde_json::to_string(packet).unwrap();
    writer.write_all(format!("{json}\n").as_bytes()).unwrap();
}

pub fn receive_packet<R: BufRead>(reader: &mut R) -> Option<Packet> {
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(0) | Err(_) => None,
        Ok(_) => serde_json::from_str(&line).ok(),
    }
}
