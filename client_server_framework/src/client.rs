use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Could not connect to server");
    let message = b"Hello, server!";
    stream.write_all(message).expect("Failed to write to server");

    let mut response = [0; 512];
    stream.read(&mut response).expect("Failed to read from server");

    println!("Received: {}", String::from_utf8_lossy(&response));
}