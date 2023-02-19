use ring::aead::{self, BoundKey};
use ring::{agreement, rand};
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
    let rng = rand::SystemRandom::new();
    let private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
    let public_key = private_key.compute_public_key().unwrap();

    let pool = ThreadPool::new(1);

    println!(
        "Server starte, listening on {}",
        listener.local_addr().unwrap()
    );
    println!("Server public key: {:?}", public_key.as_ref());

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
