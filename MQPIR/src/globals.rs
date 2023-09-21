// globals.rs
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref IP_ADDRESS: Mutex<String> = Mutex::new(String::from("127.0.0.1"));
    pub static ref PORT: Mutex<u16> = Mutex::new(8080);
    pub static ref CMD_OFFLINE_SEND_HINTS_SET: Mutex<String> = Mutex::new(String::from("offline_send_hints_set"));
    pub static ref CMD_OFFLINE_SEND_PARITIES_SET: Mutex<String> = Mutex::new(String::from("offline_send_parities_set"));
    pub static ref CMD_RECEIVE_SUCCESS: Mutex<String> = Mutex::new(String::from("receive_success"));
    pub static ref NUM_OF_HINTS: Mutex<usize> = Mutex::new(10);
    pub static ref BLOCK_SIZE: Mutex<usize> = Mutex::new(32);
    pub static ref KEY_SIZE: Mutex<usize> = Mutex::new(32);
    pub static ref N_ : Mutex<usize> = Mutex::new(1024);
}
