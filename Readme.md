# Bahn - FTP Server (RFC 959)

An FTP server project built in Rust, designed to follow RFC 959. This project serves as a learning experience for the Rust programming language.

## Features

- [x] Basic FTP server functionality
- [x] Authentication support
- [ ] File transfers (upload/download)
- [x] Initial implementation of RFC 959 commands

## Planned Features

- Support for more FTP commands (LIST, DELE, MKD, etc.)
- Virtual File System (VFS) for abstraction

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

