// client.rs
use std::io::{Read, Write};
use std::net::TcpStream;
// use toml::from_str;
// use rand::prelude::*;
use rand_core::{RngCore, OsRng};
// prepare the global variables
use crate::globals;

static mut KEYS: [u8; globals::NUM_OF_HINTS * globals::KEY_SIZE * 2] = [0; globals::NUM_OF_HINTS * globals::KEY_SIZE * 2];

pub fn preprocess() {
    let mut rng = OsRng;
    rng.fill_bytes(unsafe { &mut KEYS });

    let mut stream = TcpStream::connect(format!("{}:{}", globals::IP_ADDRESS, globals::PORT)).unwrap();
    let _ = stream.write(unsafe { &KEYS }).unwrap();

    let mut buf = [0; globals::NUM_OF_HINTS * globals::SQRT_N * globals::BLOCK_SIZE];
    let _ = stream.read_exact(&mut buf).unwrap();
    println!("Received {} bytes", buf.len());
}

pub fn search_hint() {
    let mut rng = OsRng;
    let hint_index = rng.next_u32() as usize % globals::NUM_OF_HINTS;
    for _i in 0..hint_index {
        let key = &unsafe { KEYS }[_i * globals::KEY_SIZE..(_i+1) * globals::KEY_SIZE];
        for j in 0..(2 * globals::SQRT_N) {
            let point = j.to_be_bytes();
            let _mac = crate::globals::key_mac(key, &point);
        }
    }
}
