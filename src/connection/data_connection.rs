use crate::connection::communication_channel::CommunicationChannel;
use crate::connection::data_transfer_status::DataTransferStatus;
use crate::response::ResponseCollection;
use crate::response::data::DataTransferType;
use std::io::Write;
use std::net::{Shutdown, SocketAddr, TcpListener};

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

    pub fn handle_client_connection(
        &self,
        communication_channel: CommunicationChannel<DataTransferStatus, ResponseCollection>,
    ) {
        for stream in self.connection.as_ref().unwrap().incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut transfer_status = DataTransferStatus::Success;
                    let message = communication_channel.receiver.as_ref().unwrap().recv();
                    if message.is_ok() {
                        let response = message.unwrap();
                        'response_loop: for response in response {
                            if response.data.is_some() {
                                let data = response.data.unwrap();
                                if data.transfer_type == DataTransferType::Outgoing {
                                    for content in data.content {
                                        let content = content + "\r\n";
                                        if stream.write(content.as_bytes()).is_err() {
                                            transfer_status = DataTransferStatus::Failed;
                                            break 'response_loop;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    communication_channel
                        .sender
                        .as_ref()
                        .unwrap()
                        .send(transfer_status)
                        .unwrap_or(());

                    stream.shutdown(Shutdown::Both).unwrap_or(());

                    break;
                }
                Err(_) => (),
            }
        }
    }
}
