use std::fs;
use std::io::{self, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use serde::{Deserialize, Serialize};
use decen_messenger::{receive_packet, send_packet, Packet};

#[derive(Serialize, Deserialize)]
struct Settings {
    server_ip: String,
    server_port: String,
}

fn main() {
    println!("Enter name: ");
    let name = get_input().trim().to_string();

    let settings = loop {
        match load_settings() {
            Some(s) => break s,
            None => configure_settings(),
        }
    };

    let address = format!("{}:{}", settings.server_ip, settings.server_port);
    let mut stream = TcpStream::connect(&address).unwrap();

    let mut read_stream = stream.try_clone().unwrap();
    thread::spawn(move || listen_for_packets(&mut read_stream));

    send_packet(&Packet::Join { name: name.clone() }, &mut stream);
    println!("Connected to server!");

    loop {
        print!("Enter the message: ");
        io::stdout().flush().unwrap();
        let message = get_input().trim().to_string();
        if message.is_empty() {
            continue;
        }
        send_packet(&Packet::Chat { name: name.clone(), message }, &mut stream);
    }
}

fn listen_for_packets(stream: &mut TcpStream) {
    let mut reader = BufReader::new(stream);
    loop {
        let Some(data) = receive_packet(&mut reader) else {
            println!("Lost connection with server");
            break;
        };
        match data {
            Packet::Chat { name, message } => println!("{name}: {message}"),
            Packet::Join { name } => println!("----- {name} joined -----"),
            Packet::Leave { name } => println!("----- {name} left -----"),
            _ => {}
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    input
}

fn load_settings() -> Option<Settings> {
    let data = fs::read_to_string("settings.json").ok()?;
    serde_json::from_str(&data).ok()
}

fn configure_settings() {
    print!("Would you like to set settings or use defaults? (default/yes): ");
    io::stdout().flush().unwrap();
    let decision = get_input().trim().to_lowercase();

    let (server_ip, server_port) = match decision.as_str() {
        "default" | "d" => (String::from("0.0.0.0"), String::from("2345")),
        "yes" | "y" => {
            print!("Enter the server IP: ");
            io::stdout().flush().unwrap();
            let ip = get_input().trim().to_string();
            print!("Enter the server port: ");
            io::stdout().flush().unwrap();
            let port = get_input().trim().to_string();
            (ip, port)
        }
        _ => {
            println!("Must enter 'yes' or 'default'");
            return;
        }
    };

    let settings = Settings { server_ip, server_port };
    fs::write("settings.json", serde_json::to_string(&settings).unwrap()).unwrap();
}
