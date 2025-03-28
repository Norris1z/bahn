use std::borrow::Cow;
use crate::connection::data_connection::DataConnection;
use crate::filesystem::file::representation_type::RepresentationType;
use crate::response::ResponseCollection;
use crate::session::user::User;
use std::cell::{Cell, RefCell};
use std::sync::mpsc;
use std::sync::mpsc::Sender;

pub struct CommandContext<'a> {
    user: &'a RefCell<User>,
    data_connection_created: &'a Cell<bool>,
    communication_channel: &'a RefCell<Option<Sender<ResponseCollection>>>,
}

impl<'a> CommandContext<'a> {
    pub fn new(
        user: &'a RefCell<User>,
        data_connection_created: &'a Cell<bool>,
        communication_channel: &'a RefCell<Option<Sender<ResponseCollection>>>,
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

    pub fn directory_exists(&self, path: &str) -> bool {
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

    pub fn create_data_connection(&self) -> Option<String> {
        let connection = DataConnection::new();

        if connection.connection.is_none() {
            return None;
        }

        let address = connection.get_address();

        println!("{:?}", address);

        let (sender, receiver) = mpsc::channel();

        self.communication_channel.borrow_mut().replace(sender);

        std::thread::spawn(move || connection.handle_client_connection(receiver));

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
            .unwrap().borrow().list_directory_content_names(path)
    }

    pub fn list_directory_detailed_content_information(&self, path: &Cow<str>) -> Vec<String> {
        self.user
            .borrow()
            .filesystem
            .as_ref()
            .unwrap().borrow().list_directory_detailed_content_information(path)
    }
}
