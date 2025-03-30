use crate::connection::communication_channel::CommunicationChannel;
use crate::connection::data_transfer_status::DataTransferStatus;
use crate::filesystem::VirtualFilesystem;
use crate::response::ResponseCollection;
use crate::response::data::DataTransferType;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener};
use crate::constants::DATA_CONNECTION_READ_BUFFER;

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

                                match data.transfer_type {
                                    DataTransferType::Outgoing => {
                                        for content in data.content {
                                            let content = content + "\r\n";
                                            if stream.write(content.as_bytes()).is_err() {
                                                transfer_status = DataTransferStatus::Failed;
                                                break 'response_loop;
                                            }
                                        }
                                    }
                                    DataTransferType::Incoming => {
                                        let mut buffer = [0; DATA_CONNECTION_READ_BUFFER];

                                        //TODO: refactor (currently the filename is assumed to be the first item in the vec.)
                                        let filename = data.content.first().unwrap();
                                        let writable_file =
                                            VirtualFilesystem::create_writable_file(filename);

                                        if writable_file.is_err() {
                                            transfer_status = DataTransferStatus::Failed;
                                            break 'response_loop;
                                        }

                                        let mut writable_file = writable_file.unwrap();

                                        loop {
                                            match stream.read(&mut buffer) {
                                                Ok(0) => break 'response_loop,
                                                Ok(bytes_read) => {
                                                    if writable_file
                                                        .write_all(buffer[0..bytes_read].as_ref())
                                                        .is_err()
                                                    {
                                                        transfer_status =
                                                            DataTransferStatus::Failed;
                                                        break 'response_loop;
                                                    }
                                                }
                                                Err(_) => break 'response_loop,
                                            }
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
