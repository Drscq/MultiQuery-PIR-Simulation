// server.rs
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use rand::Rng;
use crate::globals;

pub fn key_mac(key: &[u8], point: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take a key of any size");
    mac.update(point);
    mac.finalize().into_bytes().to_vec()
}


// Globals for the server

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; globals::NUM_OF_HINTS * globals::KEY_SIZE];
        let _ = stream.read_exact(&mut buf);
        for i in 0..globals::NUM_OF_HINTS {
            let key = &buf[i * globals::KEY_SIZE..(i+1) * globals::KEY_SIZE];
            for j in 0..globals::SQRT_N.lock().unwrap().clone() {
                let point = j.to_be_bytes();
                let mac = key_mac(key, &point);
                let _mac = &mac[..globals::INDEX_SIZE];
            }
        }
        let mut _rng = rand::thread_rng();
        let mut block1: [u8; globals::BLOCK_SIZE] = [0; globals::BLOCK_SIZE];
        let mut block2: [u8; globals::BLOCK_SIZE] = [0; globals::BLOCK_SIZE];
        for block in &mut block1 {
            *block = _rng.gen();
        }
        for block in &mut block2 {
            *block = _rng.gen();
        }
        let mut results = Vec::new();
        for _i in 0 .. globals::NUM_OF_HINTS {
           for _j in 0 .. globals::SQRT_N.lock().unwrap().clone() {
               for _k in 0..globals::BLOCK_SIZE {
                    results.push(block1[_k] ^ block2[_k]);
                }
            }
        }
        // Send the results to the client
        let _ = stream.write(&results);
        
}

pub fn main() {
    // Create a TCP listener on port 8080
    let listener = TcpListener::bind(format!("{}:{}", globals::IP_ADDRESS, globals::PORT)).unwrap();
    println!("Server listening on port {}...", globals::PORT);
    // Loop to accept incoming connections
    for stream in listener.incoming() {
        // Check if the connection was successful
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client
                println!("New connection from {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                // An error occurred, so print it and continue
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}