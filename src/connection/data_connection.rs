use crate::connection::{CommunicationChannel, DataTransferStatus};
use crate::constants::DATA_CONNECTION_READ_BUFFER;
use crate::filesystem::VirtualFilesystem;
use crate::response::ResponseCollection;
use crate::response::data::{DataTransferType, ResponseData, ResponseDataContentType};
use std::io::{Read, Write};
use std::net::TcpStream;

pub trait DataConnection {
    fn has_active_connection(&self) -> bool;

    fn handle_data_exchange(
        &self,
        communication_channel: CommunicationChannel<DataTransferStatus, ResponseCollection>,
    );

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
            ResponseDataContentType::File(file) => {
                if let Some(mut file_buffer) =
                    VirtualFilesystem::open_file_in_buffered_mode(&file.filename)
                {
                    let mut buffer = [0; DATA_CONNECTION_READ_BUFFER];
                    let mut write_status = None;
                    'write_loop: loop {
                        match file_buffer.read(&mut buffer) {
                            Ok(0) => break 'write_loop,
                            Ok(bytes_read) => {
                                if stream.write(&buffer[0..bytes_read]).is_err() {
                                    write_status = Some(DataTransferStatus::Failed);
                                    break 'write_loop;
                                }
                            }
                            Err(_) => {
                                write_status = Some(DataTransferStatus::Failed);
                                break 'write_loop;
                            }
                        };
                    }

                    return write_status;
                }
                Some(DataTransferStatus::Failed)
            }
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
                            if writable_file.write_all(&buffer[0..bytes_read]).is_err() {
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
}
