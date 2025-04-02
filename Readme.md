# Bahn - FTP Server (RFC 959)

An FTP server project built in Rust, designed to follow RFC 959. This project serves as a learning experience for the Rust programming language.

## Todo

- [x] Basic FTP server functionality
- [x] Authentication support
- [x] Initial implementation of RFC 959 commands
- [x] Create types for Data Transfer using the data connection, for now Response::CustomString
- [ ] Close existing PASV connection if client sends multiple PASSV to prevent Port DDOS
- [ ] Ensure user cant have a PORT and PASV command at the same time
- [ ] Use BufReader instead of calling read on the TCP stream (https://doc.rust-lang.org/std/io/struct.BufReader.html)
- [ ] Implement transmission modes according to RFC (Block, Compressed etc)
- [ ] Add debug statements for data connections (generally improve logging)
- [ ] File transfers (upload/download)
- [ ] Have another receiver in the session to signal data sent
- [ ] Use timeouts for the connections
- [ ] Add more test cases


## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- Cargo package manager

### Installation

```sh
git clone https://github.com/Norris1z/bahn.git
cd bahn
mv .env.example .env #update with preferred env
```

### Usage

```sh
cargo run
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

MIT License

