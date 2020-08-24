use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
mod mqtt;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1883").unwrap();
    for stream in listener.incoming() {
        println!("Client connected");
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    let mqtt_response = match stream.read(&mut buffer) {
        Ok(n) => mqtt::process_packet(&buffer[..n]),
        Err(_) => {
            println!("Client disconnected");
            None
        }
    };

    match mqtt_response {
        Some(resp) => println!("{:?}", resp),
        None => ()
    }
}
