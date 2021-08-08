mod client;
mod http;
mod listener;
mod router;
mod server;
use std::sync::Arc;

fn main() {
    let mut sv = server::Server::new("localhost", 8080);
    sv.run();
    sv.join();
}
