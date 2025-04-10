use crate::connection::DataConnection;
use crate::connection::communication_channel::CommunicationChannel;
use crate::connection::data_transfer_status::DataTransferStatus;
use crate::response::Response;
use std::net::{Shutdown, SocketAddr, TcpListener};

pub struct PassiveDataConnection {
    pub connection: Option<TcpListener>,
}

impl DataConnection for PassiveDataConnection {
    fn has_active_connection(&self) -> bool {
        self.connection.is_some()
    }

    fn handle_data_exchange(
        &self,
        communication_channel: CommunicationChannel<DataTransferStatus, Response>,
    ) {
        for stream in self.connection.as_ref().unwrap().incoming() {
            match stream {
                Ok(mut stream) => {
                    self.process_data_from_communication_channel(
                        &mut stream,
                        &communication_channel,
                    );

                    stream.shutdown(Shutdown::Both).unwrap_or(());

                    break;
                }
                Err(_) => (),
            }
        }
    }
}

impl PassiveDataConnection {
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

    pub fn get_address(&self) -> Option<SocketAddr> {
        if self.has_active_connection() {
            return Some(self.connection.as_ref().unwrap().local_addr().unwrap());
        }

        None
    }
}
