use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub fn start_listener(addr: &str) {
    let listener = TcpListener::bind(addr).expect("Failed to bind TCP listener");
    println!("Listening on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.write(b"Welcome to Rescape MUD! Type 'help'.\n").unwrap();

    loop {
        match stream.read(&mut buffer) {
            Ok(size) if size > 0 => {
                let command = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
                println!("Received: {}", command);
            }
            _ => break,
        }
    }
}