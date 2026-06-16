use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:2345").unwrap();

    for stream in listener.incoming() {
        println!("Client connected!");
        let mut stream = stream;

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();

        let message = String::from_utf8_lossy(&buffer[..bytes_read]);

        println!("Received: {}", message);

        stream.write_all(b"Privet from server")
    }
}
