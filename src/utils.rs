
#[derive(Serialize, Deserialize)]
enum Packet {
    Chat {
        name: String,
        message: String,
    },

    Join {
        name: String,
    },

    Leave {
        name: String,
    },

    Ping,
}


pub fn send_packet(packet: Packet, stream: &mut TcpStream) {
    let json = serde_json::to_string(&packet).unwrap();

    let _ = stream.write_all(json.as_bytes()).unwrap();
}

pub fn recieve_packet(stream: &mut TcpStream) -> Option<Packet> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    if bytes_read == 0 {
        return None;
    }

    let text = String::from(String::from_utf8_lossy(&buffer[..bytes_read]));
    serde_json::from_str(&text).unwrap()
}
