use crate::connection::communication_channel::CommunicationChannel;
use crate::connection::data_transfer_status::DataTransferStatus;
use crate::constants::DATA_CONNECTION_READ_BUFFER;
use crate::filesystem::VirtualFilesystem;
use crate::response::ResponseCollection;
use crate::response::data::{DataTransferType, ResponseData, ResponseDataContentType};
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};

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

    fn send_outgoing_data(
        &self,
        stream: &mut TcpStream,
        data: &ResponseData,
    ) -> Option<DataTransferStatus> {
        match &data.content {
            ResponseDataContentType::FileInfoList(info_list) => {
                for info in info_list {
                    let content = info.to_owned() + "\r\n";
                    if stream.write(content.as_bytes()).is_err() {
                        return Some(DataTransferStatus::Failed);
                    }
                }

                None
            }
            _ => None,
        }
    }

    fn receive_incoming_data(
        &self,
        stream: &mut TcpStream,
        data: &ResponseData,
    ) -> Option<DataTransferStatus> {
        match &data.content {
            ResponseDataContentType::File(file) => {
                let mut buffer = [0; DATA_CONNECTION_READ_BUFFER];
                let writable_file = VirtualFilesystem::create_writable_file(&file.filename);

                if writable_file.is_err() {
                    return Some(DataTransferStatus::Failed);
                }

                let mut writable_file = writable_file.unwrap();

                let mut read_result = None;

                'read_loop: loop {
                    match stream.read(&mut buffer) {
                        Ok(0) | Err(_) => break 'read_loop,
                        Ok(bytes_read) => {
                            if writable_file
                                .write_all(buffer[0..bytes_read].as_ref())
                                .is_err()
                            {
                                read_result = Some(DataTransferStatus::Failed);
                                break 'read_loop;
                            }
                        }
                    }
                }

                read_result
            }
            _ => None,
        }
    }

    fn process_data_from_communication_channel(
        &self,
        stream: &mut TcpStream,
        communication_channel: &CommunicationChannel<DataTransferStatus, ResponseCollection>,
    ) {
        let mut transfer_status = DataTransferStatus::Success;
        let message = communication_channel.receiver.as_ref().unwrap().recv();
        if message.is_ok() {
            let response = message.unwrap();
            'response_loop: for response in response {
                if response.data.is_some() {
                    let data = response.data.unwrap();

                    match data.transfer_type {
                        DataTransferType::Outgoing => {
                            if let Some(status) = self.send_outgoing_data(stream, &data) {
                                transfer_status = status;
                                break 'response_loop;
                            }
                        }
                        DataTransferType::Incoming => {
                            if let Some(status) = self.receive_incoming_data(stream, &data) {
                                transfer_status = status;
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
    }

    pub fn handle_client_connection(
        &self,
        communication_channel: CommunicationChannel<DataTransferStatus, ResponseCollection>,
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
