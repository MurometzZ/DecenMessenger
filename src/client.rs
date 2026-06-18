use std::io::Write;
use std::io;
use std::net::TcpStream;
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
    let mut stream = TcpStream::connect("0.0.0.0:2345").unwrap();
    println!("Connected to server!");

    println!("Enter name: ");
    let name = get_input();

    let packet = Packet::Join {
        name: name.clone().trim().to_string(),
    };

    let json = serde_json::to_string(&packet).unwrap();
    // println!("\nSending |{}|", json);
    let _ = stream.write_all(json.as_bytes()).unwrap();

    loop {
        println!("Enter the message: ");
        let message = get_input();

        let packet = Packet::Chat {
            name: name.clone().trim().to_string(),
            message: message.trim().to_string(),
        };

        let json = serde_json::to_string(&packet).unwrap();
        // println!("\nSending |{}|", json);

        let _ = stream.write_all(json.as_bytes()).unwrap();
    }
}

fn get_input() -> String {
    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    input
}
