use crate::connection::{CommunicationChannel, DataConnection, DataTransferStatus};
use crate::response::Response;
use socket2::{Domain, Socket, Type};
use std::net::{Shutdown, SocketAddr, TcpStream};

pub struct ActiveDataConnection {
    stream: Option<TcpStream>,
}

impl ActiveDataConnection {
    pub fn new(address: SocketAddr) -> Self {
        if let Ok(socket) = Socket::new(Domain::IPV4, Type::STREAM, None) {
            let default_address = Self::get_default_address();

            //https://stackoverflow.com/questions/14388706/how-do-so-reuseaddr-and-so-reuseport-differ
            if socket.set_reuse_address(true).is_ok()
                && socket.set_reuse_port(true).is_ok()
                && socket.bind(&default_address.into()).is_ok()
                && socket.connect(&address.into()).is_ok()
            {
                return Self {
                    stream: Some(TcpStream::from(socket)),
                };
            }
        }

        Self { stream: None }
    }

    fn get_default_address() -> SocketAddr {
        let address = dotenv::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not set");
        let port = dotenv::var("SERVER_PORT").expect("SERVER_PORT not set");

        SocketAddr::new(address.parse().unwrap(), port.parse::<u16>().unwrap() - 1)
    }
}

impl DataConnection for ActiveDataConnection {
    fn has_active_connection(&self) -> bool {
        self.stream.is_some()
    }

    fn handle_data_exchange(
        &self,
        communication_channel: CommunicationChannel<DataTransferStatus, Response>,
    ) {
        if let Ok(mut stream) = self.stream.as_ref().unwrap().try_clone() {
            self.process_data_from_communication_channel(&mut stream, &communication_channel);
        } else {
            //TODO: investigate possible bug. There might be a case where this doesnt get sent to the user PI
            communication_channel
                .sender
                .as_ref()
                .unwrap()
                .send(DataTransferStatus::Failed)
                .unwrap_or(());
        }

        self.stream
            .as_ref()
            .unwrap()
            .shutdown(Shutdown::Both)
            .unwrap_or(());
    }
}
