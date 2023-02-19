use openssl::ssl::{SslAcceptor, SslStream};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use lightningdb::ThreadPool;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    break;
                }

                let message = String::from_utf8_lossy(&buffer[..size]);
                println!("Received message: {}", message);
                stream.write_all(b"Message received\n").unwrap();
            }
            Err(e) => {
                println!("Error reading from socket: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let pool = ThreadPool::new(1);
    let acceptor = SslAcceptor::mozilla_modern_v5(openssl::ssl::SslMethod::tls()).unwrap();

    println!("Server listening on port 6379");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected: {}", stream.peer_addr().unwrap());
                pool.execute(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error connecting to client: {}", e);
            }
        }
    }
}
