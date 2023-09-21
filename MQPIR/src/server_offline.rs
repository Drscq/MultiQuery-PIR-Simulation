// server.rs
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use generic_array::GenericArray;

// type HmacSha256 = Hmac<Sha256>;

// prepare the global variables
use crate::globals;

pub fn key_mac(key: &[u8], point: &[u8]) -> Vec<u8> {
    let key = GenericArray::clone_from_slice(key);
    let mut mac = Hmac::<Sha256>::new(&key);
    mac.update(point);
    mac.finalize().into_bytes().to_vec()
}


// Globals for the server

fn handle_client(mut stream: TcpStream, size : usize) {
    // Create a buffer to store the incoming data
    let mut buf = Vec::new();
    buf.resize(size, 0);
    // Loop until the stream is closed or an error occurs
    loop {
        // Read data from the stream into the buffer
        match stream.read(&mut buf) {
            Ok(0) => {
                // No more data, so close the connection
                println!("Connection closed");
                return;
            }
            Ok(_) => {}, // Number of bytes read
            Err(e) => {
                // An error occurred, so print it and close the connection
                eprintln!("Error reading from stream: {}", e);
                return;
            }
        };
        // Write the cmd_receive_success to the stream
        let cmd_receive_success = globals::CMD_RECEIVE_SUCCESS.lock().unwrap();
        if let Err(e) = stream.write(cmd_receive_success.as_bytes()) {
            // An error occurred, so print it and close the connection
            eprintln!("Error writing to stream: {}", e);
            return;
        }
        // Convert the buffer to a string and check if it matches cmd_offline_send_hints_set in config.toml
        let buf_str = String::from_utf8_lossy(&buf);
        let cmd_offline_send_hints_set = globals::CMD_OFFLINE_SEND_HINTS_SET.lock().unwrap();
        if buf_str == *cmd_offline_send_hints_set {
            println!("Received cmd_offline_send_hints_set Successfully");
            // Ready to receive the keys
            // Create a buffer to store the incoming data  
            let mut buf = Vec::new();
            let num_of_hints = *globals::NUM_OF_HINTS.lock().unwrap();
            let key_size = *globals::KEY_SIZE.lock().unwrap();
            buf.resize(num_of_hints * key_size, 0);
            if let Err(e) = stream.read_exact(&mut buf) {
                // An error occurred, so print it and close the connection
                eprintln!("Error reading from stream: {}", e);
                return;
            }
            println!("Received keys Successfully");
            // Create a vector to store the MACs
            let mut macs = Vec::new();
            let n = *globals::N_.lock().unwrap() as f64;
            let sqrt_n = n.sqrt() as usize;
            // Use the received keys and the key_mac function
            for i in 0..num_of_hints {
                let key = &buf[i * key_size..(i+1) * key_size];
                for j in 0..sqrt_n {
                    let point = j.to_be_bytes(); // Convert the index to bytes
                    let mac = key_mac(key, &point);
                    // Store the mac
                    macs.push(mac);
                }
            }
            // Send the MACs to the client
            let flat_macs: Vec<u8> = macs.into_iter().flatten().collect();
            if let Err(e) = stream.write(&flat_macs) {
                // An error occurred, so print it and close the connection
                eprintln!("Error writing to stream: {}", e);
                return;
            }
        } else {
            println!("Received cmd_offline_send_hints_set Failed");
        }
    }
}

pub fn main() {
    // Create a TCP listener on port 8080
    let ip_address = globals::IP_ADDRESS.lock().unwrap();
    let port = globals::PORT.lock().unwrap();
    let listener = TcpListener::bind(format!("{}:{}", *ip_address, *port)).unwrap();
    println!("Server listening on port {}...", *port);
    // Loop to accept incoming connections
    for stream in listener.incoming() {
        // Check if the connection was successful
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client
                println!("New connection from {}", stream.peer_addr().unwrap());
                let cmd_offline_send_hints_set = globals::CMD_OFFLINE_SEND_HINTS_SET.lock().unwrap();
                let cmd_offline_send_hints_set_len = cmd_offline_send_hints_set.len();
                thread::spawn(move || handle_client(stream, cmd_offline_send_hints_set_len));

            }
            Err(e) => {
                // An error occurred, so print it and continue
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}