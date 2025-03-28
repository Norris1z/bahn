use crate::response::ResponseCollection;
use std::io::Write;
use std::net::{Shutdown, SocketAddr, TcpListener};
use std::sync::mpsc::Receiver;

pub struct DataConnection {
    pub connection: Option<TcpListener>,
}

impl DataConnection {
    pub fn new() -> Self {
        let listener = TcpListener::bind((
            dotenv::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not set"),
            0,
        ));

        Self {
            connection: match listener {
                Ok(listener) => Some(listener),
                Err(_) => None,
            },
        }
    }

    pub fn has_active_connection(&self) -> bool {
        self.connection.is_some()
    }

    pub fn get_address(&self) -> Option<SocketAddr> {
        if self.has_active_connection() {
            return Some(self.connection.as_ref().unwrap().local_addr().unwrap());
        }

        None
    }

    pub fn handle_client_connection(&self, receiver: Receiver<ResponseCollection>) {
        for stream in self.connection.as_ref().unwrap().incoming() {
            match stream {
                Ok(mut stream) => {
                    let message = receiver.recv();
                    if message.is_ok() {
                        let response = message.unwrap();
                        for response in response {
                            stream
                                .write_all(response.message.get_message().to_string().as_bytes())
                                .unwrap_or(()) //not sure how to handle errors in this case
                        }
                    }

                    stream.shutdown(Shutdown::Both).unwrap_or(());

                    break;
                }
                Err(_) => (),
            }
        }
    }
}
