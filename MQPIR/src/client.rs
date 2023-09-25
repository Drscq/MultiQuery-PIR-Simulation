// client.rs
use std::io::{Read, Write};
use std::net::TcpStream;
// use toml::from_str;
// use rand::prelude::*;
use rand_core::{RngCore, OsRng};
// prepare the global variables
use crate::globals;

pub fn preprocess() {

    let mut keys = [0; globals::NUM_OF_HINTS * globals::KEY_SIZE * 2];
    let mut rng = OsRng;
    rng.fill_bytes(&mut keys);

    let mut stream = TcpStream::connect(format!("{}:{}", globals::IP_ADDRESS, globals::PORT)).unwrap();
    let _ = stream.write(&keys).unwrap();

    let mut buf = [0; globals::NUM_OF_HINTS * globals::SQRT_N * globals::BLOCK_SIZE];
    let _ = stream.read_exact(&mut buf).unwrap();
    println!("Received {} bytes", buf.len());
}

