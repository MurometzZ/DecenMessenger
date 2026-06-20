mod utils;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader};
use std::sync::{Arc, Mutex};
use utils::{receive_packet, send_packet};
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
        let mut stream = stream.unwrap();
        clients.lock().unwrap().push(stream.try_clone().unwrap());

        let clients_clone = Arc::clone(&clients);

        thread::spawn(move || {
            handle_client(&mut stream, clients_clone);
        });
    }
}

fn handle_client(stream: &mut TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut reader = BufReader::new(stream);
    loop {
        let Some(data) = receive_packet(&mut reader)
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
