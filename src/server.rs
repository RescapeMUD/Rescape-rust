use crate::net::{tcp, websocket};
use tokio::runtime::Runtime;
use std::thread;

pub fn start() {
    println!("Starting Rescape MUD server...");

    // Start TCP server
    thread::spawn(|| tcp::start_listener("0.0.0.0:4000"));

    // Start WebSocket server (must use async runtime)
    thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(websocket::start_websocket("0.0.0.0:8080"));
    });

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}