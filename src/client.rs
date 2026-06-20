mod utils;
use std::io::{Read, Write};
use std::io;
use std::fs::File;
use std::net::TcpStream;
use std::thread;
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

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    server_ip: String,
    server_port: String,
}

fn main() {
    println!("Enter name: ");
    let name = get_input().trim().to_string();

    let server_ip;
    let server_port;
    loop {
        match get_settings() {
            Some(settings) => {
                println!("Got settings: ip={}, port={}", settings.server_ip, settings.server_port);
                server_ip = settings.server_ip;
                server_port = settings.server_port;
                break;
            }
            None => {
                set_settings();
            }
        }
    }

    let address = format!("{}:{}", server_ip, server_port);
    let mut stream = TcpStream::connect(address).unwrap();

    let mut read_stream = stream.try_clone().unwrap();
    thread::spawn(move || {
        listen_for_packets(&mut read_stream);
    });

    let packet = Packet::Join {
        name: name.clone(),
    };
    println!("Connected to server!");

    let json = serde_json::to_string(&packet).unwrap();
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

        send_packet(&packet, &mut stream);
    }
}

fn listen_for_packets(mut stream: &mut TcpStream) {
    loop {
        let Some(data) = recieve_packet(&mut stream)
            else {
                println!("Lost connection with server");
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

// fn send_packet(packet: Packet, stream: &mut TcpStream) {
//     let json = serde_json::to_string(&packet).unwrap();
//
//     let _ = stream.write_all(json.as_bytes()).unwrap();
// }
//
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

fn get_input() -> String {
    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    input
}

fn get_settings() -> Option<Settings> {
    let mut file = match File::open("settings.json") {
        Ok(file) => file,
        Err(_) => {
            return None // if file doesn't exist
        }
    };

    let mut data = String::new();

    match file.read_to_string(&mut data) {
        Ok(_) => {}
        Err(_) => {
            return None // if file corrupted
        }
    }

    let parsed: Result<Settings, _> = serde_json::from_str(&data);
    match parsed {
        Ok(settings) => {
            Some(settings)
        }
        Err(e) => {
            println!("Error: {}", e);

            None
        }
    }
}

fn set_settings() {
    print!("Would you like to set settings or use defaults? (Default/yes): ");
    io::stdout().flush().unwrap();
    let decision = get_input().trim().to_string().to_lowercase();

    let server_ip: String;
    let server_port: String;

    if decision == "default" || decision == "d" {
        server_ip = String::from("0.0.0.0");
        server_port = String::from("2345");
    }
    else if decision == "yes" || decision == "y" {
        print!("Enter the server IP: ");
        io::stdout().flush().unwrap();
        server_ip = get_input().trim().to_string();

        print!("Enter the server port: ");
        io::stdout().flush().unwrap();
        server_port = get_input().trim().to_string();
    }
    else {
        println!("Must enter 'yes' or 'default'");
        return;
    }

    let settings: Settings = Settings {
        server_ip,
        server_port,
    };

    let json_data = serde_json::to_string(&settings).unwrap();
    let mut file = File::create("settings.json").unwrap();
    let _ = file.write_all(json_data.as_bytes());
}
