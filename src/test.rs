use rand::prelude::*;
use rand::random;

// Assume num_of_hints and key_size are defined as constants
const num_of_hints: usize = 10;
const key_size: usize = 8;

fn main() {
    let mut keys: Vec<u8> = (0..num_of_hints * key_size).map(|_| random::<u8>()).collect();
    // print the keys
    println!("keys: {:?}", keys);
}
