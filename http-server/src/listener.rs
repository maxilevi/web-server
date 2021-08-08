use crate::http;
use crate::router::Router;
use crate::server::THREADS;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;
use threadpool::threadpool::ThreadPool;

/// Struct which listens for connections and executes the given commands.
pub struct Listener {
    pool: ThreadPool,
    addr: String,
    router: Arc<Router>,
}

impl Listener {
    pub fn new(addr: String, router: Router) -> Self {
        let pool = ThreadPool::new(THREADS);
        Listener {
            pool,
            addr,
            router: Arc::new(router),
        }
    }

    pub fn run(&self) {
        println!("Trying to bind on address {}", self.addr);
        let listener = match TcpListener::bind(&self.addr) {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to bind to socket with error: '{}'", e);
                panic!("{}", e);
            }
        };
        println!("Try Redis WEBSERVER started on address '{}'...", self.addr);

        for stream in listener.incoming() {
            let socket = stream.unwrap();
            let router = router_cpy.clone();
            self.pool.spawn(move || {
                let result = Listener::handle_connection(socket, router_cpy);
                if let Err(e) = result {
                    println!("Error whilst parsing request:\n {}", e);
                }
            });
        }
    }

    fn handle_connection(
        mut socket: TcpStream,
        handler: Arc<Router>,
    ) -> Result<(), &'static str> {
        let request_bytes = Listener::read_request_bytes(&mut socket)?;
        if request_bytes.is_empty() {
            return Ok(());
        }

        println!("Received HTTP request");
        let request_str = std::str::from_utf8(&request_bytes)
            .ok()
            .ok_or("Request was not valid UTF8")?;
        let request = http::request::Request::parse(request_str)?;
        println!("{}", request.to_string());
        let response = handler.handle(&request);

        println!("Writing response to socket...");

        let response_bytes = response.serialize();
        socket
            .write_all(&response_bytes)
            .ok()
            .ok_or("Failed to write to socket")
    }

    fn read_request_bytes(stream: &mut TcpStream) -> Result<Vec<u8>, &'static str> {
        let mut contents = Vec::new();
        let mut buffer = [0; 512];
        stream
            .set_read_timeout(Some(Duration::from_millis(1)))
            .ok()
            .ok_or("Failed to read from socket")?;
        while let Ok(r) = stream.read(&mut buffer) {
            if r == 0 {
                break;
            }
            contents.extend_from_slice(&buffer[0..r]);
        }
        Ok(contents)
    }
}
