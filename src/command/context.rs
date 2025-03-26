use crate::filesystem::file::representation_type::RepresentationType;
use crate::session::user::User;
use std::cell::RefCell;

pub struct CommandContext<'a> {
    user: &'a RefCell<User>,
}

impl<'a> CommandContext<'a> {
    pub fn new(user: &'a RefCell<User>) -> Self {
        Self { user }
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
}
