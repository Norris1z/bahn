use crate::connection::PassiveDataConnection;
use crate::connection::{ActiveDataConnection, DataTransferStatus};
use crate::connection::{CommunicationChannel, DataConnection};
use crate::filesystem::RepresentationType;
use crate::response::ResponseCollection;
use crate::session::user::User;
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::mpsc;

pub struct CommandContext<'a> {
    user: &'a RefCell<User>,
    data_connection_created: &'a Cell<bool>,
    communication_channel:
        &'a RefCell<CommunicationChannel<ResponseCollection, DataTransferStatus>>,
}

impl<'a> CommandContext<'a> {
    pub fn new(
        user: &'a RefCell<User>,
        data_connection_created: &'a Cell<bool>,
        communication_channel: &'a RefCell<
            CommunicationChannel<ResponseCollection, DataTransferStatus>,
        >,
    ) -> Self {
        Self {
            user,
            data_connection_created,
            communication_channel,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.user.borrow().is_authenticated
    }

    pub fn set_username(&self, name: String) {
        self.user.borrow_mut().username = Some(name);
    }

    pub fn get_username(&self) -> Option<String> {
        self.user.borrow().username.clone()
    }

    pub fn initialize_user_environment(&self, path: Option<String>) {
        self.user.borrow_mut().is_authenticated = true;
        self.user.borrow_mut().path = path;

        self.user.borrow_mut().setup_filesystem();
    }

    pub fn get_current_directory(&self) -> String {
        self.user
            .borrow()
            .filesystem
            .as_ref()
            .unwrap()
            .borrow()
            .get_current_directory()
    }

    pub fn file_or_directory_exists(&self, path: &str) -> bool {
        self.user
            .borrow()
            .filesystem
            .as_ref()
            .unwrap()
            .borrow()
            .exists(path)
    }

    pub fn create_directory(&self, path: &str) -> Option<String> {
        self.user
            .borrow()
            .filesystem
            .as_ref()
            .unwrap()
            .borrow()
            .create_directory(path)
    }

    pub fn change_directory(&self, path: &str) {
        self.user
            .borrow()
            .filesystem
            .as_ref()
            .unwrap()
            .borrow_mut()
            .change_directory(path)
    }

    pub fn set_representation_type(&self, representation_type: RepresentationType) {
        self.user
            .borrow()
            .filesystem
            .as_ref()
            .unwrap()
            .borrow_mut()
            .set_representation_type(representation_type)
    }

    pub fn create_passive_data_connection(&self) -> Option<String> {
        let connection = PassiveDataConnection::new();

        if !connection.has_active_connection() {
            return None;
        }

        let address = connection.get_address();

        let data_communication_channel = self.create_data_communication_channels();

        std::thread::spawn(move || connection.handle_data_exchange(data_communication_channel));

        self.data_connection_created.replace(true);

        let ip = address?.ip().to_string().replace(".", ",");
        let port = address?.port();

        Some(format!("{},{},{}", ip, port / 256, port % 256))
    }

    pub fn has_data_connection(&self) -> bool {
        self.data_connection_created.get()
    }

    pub fn list_directory_content_names(&self, path: &Cow<str>) -> Vec<String> {
        self.user
            .borrow()
            .filesystem
            .as_ref()
            .unwrap()
            .borrow()
            .list_directory_content_names(path)
    }

    pub fn list_directory_detailed_content_information(&self, path: &Cow<str>) -> Vec<String> {
        self.user
            .borrow()
            .filesystem
            .as_ref()
            .unwrap()
            .borrow()
            .list_directory_detailed_content_information(path)
    }

    pub fn delete_directory(&self, path: &str) -> bool {
        self.user
            .borrow()
            .filesystem
            .as_ref()
            .unwrap()
            .borrow()
            .delete_directory(path)
    }

    pub fn get_relative_path(&self, path: &str) -> String {
        self.user
            .borrow()
            .filesystem
            .as_ref()
            .unwrap()
            .borrow()
            .get_relative_path(path)
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn construct_socket_addr(&self, address: &str) -> Option<SocketAddr> {
        if let Some(segments) = address.split(",").collect::<Vec<&str>>().get(..6) {
            if let (Some(h1), Some(h2), Some(h3), Some(h4), Some(p1), Some(p2)) = (
                segments.get(0).and_then(|s| s.parse::<u8>().ok()),
                segments.get(1).and_then(|s| s.parse::<u8>().ok()),
                segments.get(2).and_then(|s| s.parse::<u8>().ok()),
                segments.get(3).and_then(|s| s.parse::<u8>().ok()),
                segments.get(4).and_then(|s| s.parse::<u8>().ok()),
                segments.get(5).and_then(|s| s.parse::<u8>().ok()),
            ) {
                let port = p1 as u16 * 256 + p2 as u16;
                return Some(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(h1, h2, h3, h4)),
                    port,
                ));
            }
        }

        None
    }

    fn create_data_communication_channels(
        &self,
    ) -> CommunicationChannel<DataTransferStatus, ResponseCollection> {
        let (session_sender, data_receiver) = mpsc::channel();
        let (data_sender, session_receiver) = mpsc::channel();

        let mut session_channel = self.communication_channel.borrow_mut();
        *session_channel = CommunicationChannel::new(Some(session_sender), Some(session_receiver));

        CommunicationChannel::new(Some(data_sender), Some(data_receiver))
    }

    pub fn create_active_data_connection(&self, address: SocketAddr) -> bool {
        let connection = ActiveDataConnection::new(address);

        if !connection.has_active_connection() {
            return false;
        }

        let data_communication_channel = self.create_data_communication_channels();

        std::thread::spawn(move || connection.handle_data_exchange(data_communication_channel));

        self.data_connection_created.replace(true);

        true
    }
}
