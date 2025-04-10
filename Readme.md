# Bahn - FTP Server (RFC 959)

An FTP server project built in Rust, designed to follow RFC 959. This project serves as a learning experience for the Rust programming language.

## Todo

- [x] Basic FTP server functionality
- [x] Authentication support
- [x] Initial implementation of RFC 959 commands
- [x] Create types for Data Transfer using the data connection, for now Response::CustomString
- [x] Close existing PASV connection if client sends multiple PASSV to prevent Port DDOS
- [x] File transfers (upload/download)
- [x] Have another receiver in the session to signal data sent
- [x] Ensure user cant have a PORT and PASV command at the same time
- [ ] Refactor the Response into maybe an Enum or something else since the data connection usually doesn't care about the code and message
- [ ] Use BufReader instead of calling read on the TCP stream (https://doc.rust-lang.org/std/io/struct.BufReader.html)
- [ ] Implement transmission modes according to RFC (Block, Compressed etc)
- [ ] Add debug statements for data connections (generally improve logging)
- [ ] Use timeouts for the connections
- [ ] Add more test cases

## FTP Commands According to RFC959

| Command | Arguments | Implemented |
|---------|-----------|-------------|
| USER    | `<username>` | [x]         |
| PASS    | `<password>` | [x]         |
| ACCT    | `<account-information>` | [ ]         |
| CWD     | `<pathname>` | [x]         |
| CDUP    | *(none)* | [x]         |
| SMNT    | `<pathname>` | [ ]         |
| QUIT    | *(none)* | [x]         |
| REIN    | *(none)* | [x]         |
| PORT    | `<host-port>` | [x]         |
| PASV    | *(none)* | [x]         |
| TYPE    | `<type-code>` | [x]         |
| STRU    | `<structure-code>` | [ ]         |
| MODE    | `<mode-code>` | [ ]         |
| RETR    | `<pathname>` | [x]         |
| STOR    | `<pathname>` | [x]         |
| STOU    | *(none)* | [x]         |
| APPE    | `<pathname>` | [ ]         |
| ALLO    | `<decimal-integer> [ R <decimal-integer> ]` | [ ]         |
| REST    | `<marker>` | [ ]         |
| RNFR    | `<pathname>` | [ ]         |
| RNTO    | `<pathname>` | [ ]         |
| ABOR    | *(none)* | [ ]         |
| DELE    | `<pathname>` | [x]         |
| RMD     | `<pathname>` | [x]         |
| MKD     | `<pathname>` | [x]         |
| PWD     | *(none)* | [x]         |
| LIST    | `[<pathname>]` | [x]         |
| NLST    | `[<pathname>]` | [x]         |
| SITE    | `<string>` | [ ]         |
| SYST    | *(none)* | [x]         |
| STAT    | `[<pathname>]` | [ ]         |
| HELP    | `[<string>]` | [x]         |
| NOOP    | *(none)* | [x]         |


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

