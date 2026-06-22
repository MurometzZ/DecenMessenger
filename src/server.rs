use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use decen_messenger::{receive_packet, send_packet, Packet};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:2345").unwrap();
    println!("Server started");

    let clients = Arc::new(Mutex::new(Vec::<TcpStream>::new()));

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        clients.lock().unwrap().push(stream.try_clone().unwrap());
        let clients = Arc::clone(&clients);
        thread::spawn(move || handle_client(&mut stream, clients));
    }
}

fn handle_client(stream: &mut TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let sender_address = stream.peer_addr().unwrap();
    let mut reader = BufReader::new(stream);
    loop {
        let Some(data) = receive_packet(&mut reader) else {
            println!("Client disconnected");
            break;
        };

        match &data {
            Packet::Chat { name, message } => println!("{name}: {message}"),
            Packet::Join { name } => println!("----- {name} joined -----"),
            Packet::Leave { name } => println!("----- {name} left -----"),
            _ => {}
        }

        // NOTE: Comparing by address might not be best way to compare, might need to rework
        for client in clients.lock().unwrap().iter_mut() {
            // compare sender and receiver to not send back packet to sender
            if client.peer_addr().unwrap() != sender_address {
                send_packet(&data, client);
            }
        }
    }
}
