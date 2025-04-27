mod server;
mod game;
mod net;
mod storage;

fn main() {
    println!("Starting Rescape MUD server...");
    server::start();
}
