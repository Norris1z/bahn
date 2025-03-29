use crate::filesystem::VirtualFilesystem;
use std::cell::RefCell;

pub struct User {
    pub is_authenticated: bool,
    pub username: Option<String>,
    pub path: Option<String>,
    pub filesystem: Option<RefCell<VirtualFilesystem>>,
}

impl User {
    pub fn new() -> Self {
        Self {
            is_authenticated: false,
            username: None,
            path: None,
            filesystem: None,
        }
    }

    pub fn setup_filesystem(&mut self) {
        let filesystem = VirtualFilesystem::new(self.path.clone().unwrap());
        filesystem.setup();

        self.filesystem = Some(RefCell::new(filesystem));
    }
}
