use std::net::TcpListener;

fn main() {
    let port: i16 = 2345;

    println!("Starting program...");

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
    }
}
