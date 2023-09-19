// server.rs
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use toml::from_str;

fn handle_client(mut stream: TcpStream, mut size : usize) {
    // Create a buffer to store the incoming data
    let mut buf = Vec::new();
    buf.resize(size, 0);
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

pub fn main() {
    // Create a TCP listener on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on port 8080");
    // Loop to accept incoming connections
    for stream in listener.incoming() {
        // Check if the connection was successful
        match stream {
            Ok(mut stream) => {
                // Spawn a new thread to handle the client
                println!("New connection from {}", stream.peer_addr().unwrap());
                let cmd_offline_send_hints_set = "offline_send_hints_set";
                thread::spawn(|| handle_client(stream, cmd_offline_send_hints_set.len()));
                // // receive the rely from the client to check whether it is cmd_offline_send_hints_set in config.toml or not
                // // let config: toml::Value = from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap();
                // let cmd_offline_send_hints_set = "offline_send_hints_set";
                // // Create a buffer to store the response from the client
                // let mut buf = Vec::new();
                
                // // Read the response from the stream into the buffer
                // let bytes_read = match stream.read(&mut buf) {
                //     Ok(0) => {
                //         // No more data, so close the connection
                //         println!("Connection closed");
                //         return;
                //     }
                //     Ok(n) => n, // Number of bytes read
                //     Err(e) => {
                //         // An error occurred, so print it and close the connection
                //         eprintln!("Error reading from stream: {}", e);
                //         return;
                //     }
                // };
                // // Check if the response matches the original message
                // assert_eq!(cmd_offline_send_hints_set.as_bytes(), &buf);
                // // send the rely to the client to check whether it is cmd_receive_success in config.toml or not
                // let cmd_receive_success = "receive_success";
                // // Write the message to the stream
                // if let Err(e) = stream.write(cmd_receive_success.as_bytes()) {
                //     // An error occurred, so print it and exit
                //     eprintln!("Error writing to stream: {}", e);
                //     return;
                // }
            }
            Err(e) => {
                // An error occurred, so print it and continue
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}