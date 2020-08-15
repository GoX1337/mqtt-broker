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
    let mut buffer = Vec::new(); 
    match stream.read_to_end(&mut buffer) {
        Ok(result) => {
            println!("Request: {:?}, result: {}", buffer, result);
            let packet = mqtt::MqttPacket::new(buffer);
            println!("{:?}", packet);
        },
        Err(_) => println!("Client disconnected")
    }
}
