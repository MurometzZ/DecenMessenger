use std::io::Read;
use std::net::{TcpListener, TcpStream};
extern crate serde;
use serde::{Serialize, Deserialize};

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

fn main() {
    let stream = TcpListener::bind("0.0.0.0:2345").unwrap();
    println!("Server started");

    for connection in stream.incoming() {
        handle_client(connection.unwrap());
    }
}
// TODO Add async to support multiple clients
fn handle_client(mut stream: TcpStream) {
    loop {
        let Some(data) = recieve_data(&mut stream)
        else {
            println!("Client disconnected");
            break;
        };

        match data {
            Packet::Chat {name, message} => {
                println!("{}: {}", name, message);
            }
            Packet::Join {name} => {
                println!("----- {} joined the chat -----", name);
            }
            Packet::Leave {name} => {
                println!("----- {} left the chat -----", name);
            }
            _ => {
                println!("Unexpected packet");
            }
        }
    }
}

fn recieve_data(stream: &mut TcpStream) -> Option<Packet> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    if bytes_read == 0 {
        return None;
    }

    let text = String::from(String::from_utf8_lossy(&buffer[..bytes_read]));
    serde_json::from_str(&text).unwrap()
}
