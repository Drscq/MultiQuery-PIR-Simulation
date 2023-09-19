// server.rs
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // Create a buffer to store the incoming data
    let mut buf = [0; 512];
    // Loop until the stream is closed or an error occurs
    loop {
        // Read data from the stream into the buffer
        let bytes_read = match stream.read(&mut buf) {
            Ok(0) => {
                // No more data, so close the connection
                println!("Connection closed");
                return;
            }
            Ok(n) => n, // Number of bytes read
            Err(e) => {
                // An error occurred, so print it and close the connection
                eprintln!("Error reading from stream: {}", e);
                return;
            }
        };
        // Write the same data back to the stream
        if let Err(e) = stream.write(&buf[..bytes_read]) {
            // An error occurred, so print it and close the connection
            eprintln!("Error writing to stream: {}", e);
            return;
        }
    }
}

fn main() {
    // Create a TCP listener on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on port 8080");
    // Loop to accept incoming connections
    for stream in listener.incoming() {
        // Check if the connection was successful
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client
                println!("New connection from {}", stream.peer_addr().unwrap());
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                // An error occurred, so print it and continue
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}