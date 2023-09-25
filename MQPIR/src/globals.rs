// globals.rs
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

pub const IP_ADDRESS: &str = "127.0.0.1";
pub const PORT: &str = "8080";
pub const NUM_OF_HINTS: usize = 10;
pub const BLOCK_SIZE: usize = 4096;
pub const KEY_SIZE: usize = 32;
// pub const N: usize = 1024;
pub const SQRT_N: usize = 32;
pub const INDEX_SIZE: usize = 2;

// public functions
pub fn key_mac(key: &[u8], point: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take a key of any size");
    mac.update(point);
    mac.finalize().into_bytes().to_vec()
}