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

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    server_address: String,
    server_port: String,
}

fn main() {
    let mut stream = TcpStream::connect("0.0.0.0:2345").unwrap();
    println!("Connected to server!");

    println!("Enter name: ");
    let name = get_input().trim().to_string();

    let packet = Packet::Join {
        name: name.clone(),
    };

    let json = serde_json::to_string(&packet).unwrap();
    // println!("\nSending |{}|", json);
    let _ = stream.write_all(json.as_bytes()).unwrap();

    loop {
        print!("Enter the message: ");
        io::stdout().flush().unwrap();
        let message = get_input().trim().to_string();

        if message == "" {
            continue;
        }

        let packet = Packet::Chat {
            name: name.clone(),
            message: message,
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
