mod commands;
mod db;
use commands::{Command, CommandType};
use db::DB;
use lightningdb::ThreadPool;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

fn execute(command: &Command, db: Arc<DB>) -> Result<String, &'static str> {
    match command.command {
        CommandType::STRINGSGET => {
            println!("GET command");
            if command.args.len() != 1 {
                return Err("Too many arguments");
            }
            if let Some(key) = command.args.get(0) {
                let result = db.strings_get(key.as_str());
                match result {
                    Ok(result) => return Ok(result),
                    Err(e) => return Err(e),
                }
            } else {
                return Err("Internal error");
            }
        }

        CommandType::STRINGSSET => {
            println!("SET command");
            if command.args.len() != 2 {
                return Err("Not enough arguments\nShould follow the convention `SET key value`");
            }

            let key = command.args[0].as_str();
            let value = command.args[1].as_str();
            let result = db.strings_set(key, value);
            match result {
                // Old value replaced
                Some(_) => return Ok("Ok".to_string()),
                // New value created
                None => return Ok("Ok".to_string()),
            }
        }
        CommandType::STRINGSDEL => {
            println!("DEL command");
            if command.args.len() != 1 {
                return Err("Too many arguments");
            }
            let key = command.args[0].as_str();
            match db.strings_del(key) {
                Some((removed_key, removed_value)) => {
                    return Ok(format!("{} : {}", removed_key, removed_value))
                }
                None => return Ok("No key found".to_string()),
            }
        }
        CommandType::STRINGGETSET => {
            println!("GETSET command");
            if command.args.len() != 2 {
                return Err("Not enough arguments\nShould follow the convention `SET key value`");
            }

            let key = command.args[0].as_str();
            let value = command.args[1].as_str();
            let result = db.strings_set(key, value);
            match result {
                // Old value replaced
                Some(old_value) => return Ok(format!("{}", old_value)),
                // New value created
                None => return Ok("No old value was found".to_string()),
            }
        }
        CommandType::STRINGGETDEL => {
            println!("GETDEL command");
            if command.args.len() != 1 {
                return Err("Too many arguments");
            }
            let key = command.args[0].as_str();
            match db.strings_del(key) {
                Some((_, removed_value)) => return Ok(format!("{}", removed_value)),
                None => return Ok("No key found".to_string()),
            }
        }
        CommandType::STRINGAPPEND => {
            println!("SET command");
            if command.args.len() != 2 {
                return Err("Not enough arguments\nShould follow the convention `SET key value`");
            }

            let key = command.args[0].as_str();
            let value = command.args[1].as_str();
            let result = db.strings_append(key, value);
            match result {
                // Old value replaced
                Some(_) => return Ok("Ok".to_string()),
                // New value created
                None => return Ok("Ok".to_string()),
            }
        }
        CommandType::STRINGSTRLEN => {
            println!("GET command");
            if command.args.len() != 1 {
                return Err("Too many arguments");
            }
            if let Some(key) = command.args.get(0) {
                let result = db.strings_len(key.as_str());
                match result {
                    Some(len) => return Ok(format!("{}", len)),
                    None => return Err("Key does not exist"),
                }
            } else {
                return Err("Internal error");
            }
        }
        CommandType::SAVE => {
            println!("SAVE command");
            return Ok("Ok".to_string());
        }
        CommandType::AUTH => {
            return Ok("Auth".to_string());
        }
    }
}

fn handle_client(mut stream: TcpStream, db: Arc<DB>) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    break;
                }

                let command_str = String::from_utf8_lossy(&buffer[..size]);
                let command = Command::new(&command_str);
                match command {
                    Ok(command) => {
                        let result = execute(&command, Arc::clone(&db));
                        match result {
                            Ok(message) => stream.write_all(message.as_bytes()).unwrap(),
                            Err(e) => stream.write_all(e.as_bytes()).unwrap(),
                        }
                        continue;
                    }
                    Err(e) => {
                        stream.write_all(e.as_bytes()).unwrap();
                        continue;
                    }
                }
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
    let db = Arc::new(DB::new());

    println!(
        "Server starte, listening on {}",
        listener.local_addr().unwrap()
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected: {}", stream.peer_addr().unwrap());
                // Make a clone of the Arc pointer
                let db = Arc::clone(&db);
                // We use the `move` keyword to force a closure to take ownership of the values it uses
                pool.execute(move || {
                    handle_client(stream, db);
                });
            }
            Err(e) => {
                println!("Error connecting to client: {}", e);
            }
        }
    }
}
