// client.rs
use std::io::{Read, Write};
use std::net::TcpStream;
// use toml::from_str;
use rand::prelude::*;
// prepare the global variables
use crate::globals;

pub fn preprocess() {
    let mut keys = [0; globals::NUM_OF_HINTS * globals::KEY_SIZE];
    for i in 0..globals::NUM_OF_HINTS {
        let mut rng = rand::thread_rng();
        rng.fill(&mut keys[i * globals::KEY_SIZE..(i + 1) * globals::KEY_SIZE]);
    }

    let mut stream = TcpStream::connect(format!("{}:{}", globals::IP_ADDRESS, globals::PORT)).unwrap();
    let _ = stream.write(&keys).unwrap();

    let mut buf = [0; globals::NUM_OF_HINTS * globals::SQRT_N];
    let _ = stream.read_exact(&mut buf).unwrap();
    println!("Received {} bytes", buf.len());
}

