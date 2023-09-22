// globals.rs
use std::sync::Mutex;
use lazy_static::lazy_static;

pub const IP_ADDRESS: &str = "127.0.0.1";
pub const PORT: &str = "8080";
pub const NUM_OF_HINTS: usize = 10;
pub const BLOCK_SIZE: usize = 32;
pub const KEY_SIZE: usize = 32;
pub const N: usize = 1024;
lazy_static! {
    pub static ref SQRT_N: Mutex<usize> = Mutex::new((N as f64).sqrt() as usize);
}
pub const INDEX_SIZE: usize = 2;