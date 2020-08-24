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
    match stream.read(&mut buffer) {
        Ok(n) => {
            let packet = mqtt::packet::MqttPacket::new(&buffer[..n]);
            println!("{:?}", packet);
        },
        Err(_) => println!("Client disconnected")
    }
}
