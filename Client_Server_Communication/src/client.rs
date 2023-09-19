// client.rs
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // Connect to the server on port 8080
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connected to server");
    // Create a message to send to the server
    let msg = b"Hello, world!";
    // Write the message to the stream
    if let Err(e) = stream.write(msg) {
        // An error occurred, so print it and exit
        eprintln!("Error writing to stream: {}", e);
        return;
    }
    println!("Sent message: {:?}", msg);
    // Create a buffer to store the response from the server
    let mut buf = [0; msg.len()];
    // Read the response from the stream into the buffer
    if let Err(e) = stream.read_exact(&mut buf) {
        // An error occurred, so print it and exit
        eprintln!("Error reading from stream: {}", e);
        return;
    }
    println!("Received response: {:?}", buf);
    // Check if the response matches the original message
    assert_eq!(msg, &buf);
    println!("Echo successful");
}