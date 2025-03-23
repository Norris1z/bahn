use std::fs;
use std::path::{MAIN_SEPARATOR, Path};

pub struct VirtualFilesystem {
    mount_point: String,
    home_directory: String,
    current_directory: String,
}

impl VirtualFilesystem {
    pub fn new(home_directory: String) -> Self {
        Self {
            mount_point: dotenv::var("STORAGE_MOUNT_PATH").expect("STORAGE_MOUNT_PATH not set"),
            home_directory,
            current_directory: String::from("/"),
        }
    }

    pub fn setup(&self) {
        let home_directory = Path::new(self.home_directory.as_str())
            .strip_prefix(MAIN_SEPARATOR.to_string())
            .unwrap();

        let path = Path::new(self.mount_point.as_str()).join(home_directory);

        fs::create_dir_all(path).unwrap();
    }

    pub fn get_current_directory(&self) -> String {
        self.current_directory.clone()
    }
}
