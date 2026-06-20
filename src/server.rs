mod utils;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use utils::{recieve_packet, send_packet};
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
    let connection = TcpListener::bind("0.0.0.0:2345").unwrap();
    println!("Server started");

    let clients = Arc::new(Mutex::new(Vec::<TcpStream>::new()));

    for stream in connection.incoming() {
        let stream = stream.unwrap();
        clients.lock().unwrap().push(stream.try_clone().unwrap());

        let clients_clone = Arc::clone(&clients);

        thread::spawn(move || {
            handle_client(stream, clients_clone);
        });
    }
}

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    loop {
        let Some(data) = recieve_packet(&mut stream)
            else {
                println!("Client disconnected");
                break;
            };

        match &data {
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

        let mut clients = clients.lock().unwrap();

        for client in clients.iter_mut() {
            send_packet(&data, client);
        }
    }
}

// fn recieve_packet(stream: &mut TcpStream) -> Option<Packet> {
//     let mut buffer = [0; 1024];
//     let bytes_read = stream.read(&mut buffer).unwrap();
//
//     if bytes_read == 0 {
//         return None;
//     }
//
//     let text = String::from(String::from_utf8_lossy(&buffer[..bytes_read]));
//     serde_json::from_str(&text).unwrap()
// }
//
// fn send_packet(packet: &Packet, stream: &mut TcpStream) {
//     let json = serde_json::to_string(&packet).unwrap();
//
//     let _ = stream.write_all(json.as_bytes()).unwrap();
// }
