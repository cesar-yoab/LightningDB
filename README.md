<p align="center">
   <img src="https://raw.githubusercontent.com/cesar-yoab/LightningDB/main/.github/LightningDb.png" width="200">
</p>

# LightningDB: A Redis-like In-Memory Database Written in Rust
![image](https://img.shields.io/badge/license-GPL--3.0-blue)
![image](https://img.shields.io/badge/build-passing-green)
![image](https://img.shields.io/badge/docker--build-passing-green)

LightningDB is a hobby project that aims to replicate the functionality of the popular Redis database using the Rust programming language. This project provides an in-memory key-value store that can be used as a simple caching layer, or as a data structure server for complex applications.

## Features

LightningDB supports the following features:

- In-memory key-value store: All data is stored in RAM, making it ideal for fast read/write operations.
- String data type: Key-value pairs can store any string value.
- Basic operations: Supports basic Redis operations like SET, GET, and DEL.
- Persistence: Supports persistence to disk through serialization and deserialization of data.

## Getting Started

### 1. Building from source

#### Requirements

- Rust programming language if you wish to build the project from source

#### Build
1. Clone the repository:
```bash
git clone https://github.com/cesar-yoab/LightningDB/
```

2. Build the project
```bash
cd LightningDB/lightningdb
cargo build --release
```
3. Run the server:
```bash
cargo run --release
```

4. Connect to the server using the provided python client
```bash
python lightningdb-cli
```

## Usage
- `SET key value`: Set the value of a key.
- `GET key`: Get the value of a key
- `DEL key`: Delete a key-value pair
- `SAVE`: Save the current data to disk

## Performance
Rust-Redis is designed to be fast and efficient, with performance comparable to Redis. Some initial performance benchmarks show that Rust-Redis can handle thousands of requests per second with low latency.

## License
This project is licensed under the GPL-3.0 License - see the [license](https://github.com/cesar-yoab/LightningDB/blob/main/LICENSE)
