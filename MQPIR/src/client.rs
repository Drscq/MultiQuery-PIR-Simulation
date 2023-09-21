// client.rs
use std::io::{Read, Write};
use std::net::TcpStream;
// use toml::from_str;
use rand::prelude::*;
// prepare the global variables
use crate::globals;
// preprocess function:
// 1. randomly generate a set of dummy keys: num_of_hints * key_size 
// 2. send the keys to the server
// 3. receive the parties from the server
pub fn preprocess(num_of_hints: usize, _block_size: usize, key_size: usize) -> Vec<u8> {
    // randomly generate a mutable array of size num_of_hints * key_size
    let keys: Vec<u8> = (0..num_of_hints * key_size).map(|_| random::<u8>()).collect();
    // Connect to the server on port 8080 and send the keys to the server
    // read the config.toml
    // let config: toml::Value = from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap();
    let ip_address = globals::IP_ADDRESS.lock().unwrap();
    let port = globals::PORT.lock().unwrap();
    let mut stream = TcpStream::connect(format!("{}:{}", *ip_address, *port)).unwrap();
    println!("Connected to server");
    // first send the command of cmd_offline_send_hints_set in config.toml
    let cmd_offline_send_hints_set = globals::CMD_OFFLINE_SEND_HINTS_SET.lock().unwrap();
    // Write the message to the stream
    if let Err(e) = stream.write(cmd_offline_send_hints_set.as_bytes()) {
        // An error occurred, so print it and exit
        eprintln!("Error writing to stream: {}", e);
        return keys;
    }
    // receive the rely from the server to check whether it is cmd_receive_success in config.toml or not
    // let cmd_receive_success = "receive_success";
    // Create a buffer to store the response from the server
    let cmd_receive_success = globals::CMD_RECEIVE_SUCCESS.lock().unwrap().clone();
    let mut buf = Vec::new();
    buf.resize(cmd_receive_success.len(), 0);
    // Read the response from the stream into the buffer
    if let Err(e) = stream.read_exact(&mut buf) {
        // An error occurred, so print it and exit
        eprintln!("Error reading from stream: {}", e);
        return keys;
    }
    // Convert the buffer to a string
    let buf_str = String::from_utf8_lossy(&buf);
    // Check if the response matches the original message
    assert_eq!(cmd_receive_success, buf_str);

    // Write the keys to the stream
    if let Err(e) = stream.write(&keys) {
        // An error occurred, so print it and exit
        eprintln!("Error writing to stream: {}", e);
        return keys;
    }
    println!("Sent keys Successfully");
    // Create a buffer with size num_of_hints * sqrtN_ * mac_size to store the MACs(parties) from the server
    let n = *globals::N_.lock().unwrap() as f64;
    let sqrt_n = n.sqrt() as usize;
    let mac_size = 32;
    let mut buf = Vec::new();
    buf.resize(num_of_hints * sqrt_n * mac_size, 0);
    // Read the response from the stream into the buffer
    match stream.read_exact(&mut buf) {
        Ok(_) => {
            println!("Received parties Successfully");
        },
        Err(e) => {
            // An error occurred, so print it and exit
            eprintln!("Error reading from stream: {}", e);
            return keys;
        }
    };

    return keys;
} 

// fn main() {
//     // Connect to the server on port 8080
//     let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
//     println!("Connected to server");
//     // Create a message to send to the server
//     const MSG: &[u8] = b"Hello, world!";
//     // Write the message to the stream
//     if let Err(e) = stream.write(MSG) {
//         // An error occurred, so print it and exit
//         eprintln!("Error writing to stream: {}", e);
//         return;
//     }
//     println!("Sent message: {:?}", MSG);
//     // Create a buffer to store the response from the server
//     let mut buf = [0; MSG.len()];
//     // Read the response from the stream into the buffer
//     if let Err(e) = stream.read_exact(&mut buf) {
//         // An error occurred, so print it and exit
//         eprintln!("Error reading from stream: {}", e);
//         return;
//     }
//     println!("Received response: {:?}", buf);
//     // Check if the response matches the original message
//     assert_eq!(MSG, &buf);
//     println!("Echo successful");
// }