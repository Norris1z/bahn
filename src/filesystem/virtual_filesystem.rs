use crate::filesystem::file::representation_type::RepresentationType;
use std::borrow::Cow;
use std::fs;
use std::path::{Component, MAIN_SEPARATOR, PathBuf};

pub struct VirtualFilesystem {
    mount_point: String,
    home_directory: String,
    current_directory: String,
    representation_type: Option<RepresentationType>,
}

impl VirtualFilesystem {
    pub fn new(home_directory: String) -> Self {
        Self {
            mount_point: dotenv::var("STORAGE_MOUNT_PATH").expect("STORAGE_MOUNT_PATH not set"),
            home_directory: VirtualFilesystem::trim_leading_slash(&home_directory),
            current_directory: String::from("/"),
            representation_type: None,
        }
    }

    pub fn setup(&self) {
        fs::create_dir_all(self.get_relative_path("")).unwrap();
    }

    pub fn get_current_directory(&self) -> String {
        self.current_directory.clone()
    }

    fn trim_leading_slash(path: &str) -> String {
        path.trim_start_matches(MAIN_SEPARATOR).to_string()
    }

    fn canonicalize_path(&self, path: &str) -> PathBuf {
        let mut path_buffer = PathBuf::from(self.current_directory.as_str());

        if !path.is_empty() {
            for component in PathBuf::from(path).components() {
                match component {
                    Component::ParentDir => {
                        path_buffer.pop();
                    }
                    Component::Normal(part) => path_buffer.push(part),
                    _ => {}
                }
            }
        }

        path_buffer
    }
    fn get_relative_path(&self, path: &str) -> PathBuf {
        let mut path_buffer = PathBuf::from(self.mount_point.as_str());
        path_buffer.push(self.home_directory.as_str());

        path_buffer.push(VirtualFilesystem::trim_leading_slash(
            self.canonicalize_path(path).to_str().unwrap(),
        ));

        path_buffer
    }

    pub fn exists(&self, path: &str) -> bool {
        self.get_relative_path(path)
            .try_exists()
            .unwrap_or_else(|_| false)
    }

    pub fn create_directory(&self, path: &str) -> Option<String> {
        let resource = self.get_relative_path(path);

        match fs::create_dir(resource) {
            Ok(_) => Some(self.canonicalize_path(path).to_str()?.to_string()),
            Err(_) => None,
        }
    }

    pub fn change_directory(&mut self, path: &str) {
        self.current_directory = self.canonicalize_path(path).to_str().unwrap().to_string();
    }

    pub fn set_representation_type(&mut self, representation_type: RepresentationType) {
        self.representation_type = Some(representation_type)
    }

    pub fn list_directory_content_names(&self, path: &Cow<str>) -> Vec<String> {
        let path = self.canonicalize_path(path.as_ref());
        let directory = path.to_str().unwrap();

        let mut content = vec![];

        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries {
                if let Ok(entry) = entry {
                    content.push(entry.file_name().to_str().unwrap().to_string());
                }
            }
        }

        content
    }
}
