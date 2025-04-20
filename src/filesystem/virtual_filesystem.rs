use crate::constants;
use crate::constants::{
    ASSUMED_SIX_MONTHS_IN_DAYS, FILE_TYPE_MASK, GROUP_PERMISSIONS_MASK, OTHERS_PERMISSIONS_MASK,
    OWNER_PERMISSIONS_MASK,
};
use crate::filesystem::file::representation_type::RepresentationType;
use chrono::{DateTime, Duration, Local};
use std::borrow::Cow;
use std::fs;
use std::fs::{File, Metadata};
use std::io::BufReader;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
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
        let main_separator_str = MAIN_SEPARATOR.to_string();

        let mut path_buffer = PathBuf::from(if path.starts_with(MAIN_SEPARATOR) {
            main_separator_str.as_str()
        } else {
            self.current_directory.as_str()
        });

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

    pub fn get_relative_path(&self, path: &str) -> PathBuf {
        let mut path_buffer = PathBuf::from(self.mount_point.as_str());
        path_buffer.push(self.home_directory.as_str());

        path_buffer.push(VirtualFilesystem::trim_leading_slash(
            self.canonicalize_path(path).to_str().unwrap(),
        ));

        path_buffer
    }

    pub fn directory_exists(&self, path: &str) -> bool {
        let path = self.get_relative_path(path);

        path.is_dir() && path.try_exists().unwrap_or_else(|_| false)
    }

    pub fn file_exists(&self, path: &str) -> bool {
        let path = self.get_relative_path(path);

        path.is_file() && path.try_exists().unwrap_or_else(|_| false)
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
        let path = self.get_relative_path(path.as_ref());
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

    pub fn list_directory_detailed_content_information(&self, path: &Cow<str>) -> Vec<String> {
        let path = self.get_relative_path(path.as_ref());
        let directory = path.to_str().unwrap();

        let mut content = vec![];

        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        content.push(
                            self.parse_file_metadata(entry.file_name().to_str().unwrap(), metadata),
                        )
                    }
                }
            }
        }

        content
    }

    pub fn delete_directory(&self, path: &str) -> bool {
        let path = self.get_relative_path(path.as_ref());

        fs::remove_dir_all(path).is_ok()
    }

    //TODO: figure out how to handle owner and group
    fn parse_file_metadata(&self, filename: &str, metadata: Metadata) -> String {
        let last_modified: DateTime<Local> = metadata.modified().unwrap().into();
        let format = if last_modified < Local::now() - Duration::days(ASSUMED_SIX_MONTHS_IN_DAYS) {
            "%b %d %Y"
        } else {
            "%b %d %H:%M"
        };

        format!(
            "{} {} bahn bahn {:12} {} {}",
            self.format_permissions(metadata.permissions().mode()),
            metadata.nlink(),
            metadata.len(),
            last_modified.format(format),
            filename
        )
    }

    /// This method formats the file permissions based on the raw `mode` value.
    /// The `mode` represents the file mode, which includes both the **file type** and the **permissions**.
    ///
    /// The file mode is a 16-bit value (or larger, depending on the system) that combines the file type information
    /// with the read, write, and execute permissions for the file owner, group, and others. The lower 9 bits of the mode
    /// represent the permissions (3 bits each for owner, group, and others), while the higher bits represent the file type.
    ///
    /// To extract the **file type**, we use a bitmask (`0o170000`) to isolate the top bits. This allows us to determine whether
    /// the file is a **regular file**, **directory**, **symlink**, etc. The reason we need to shift the mode bits is because
    /// the **file type** is stored in the upper bits (usually the top 4 or 5 bits, depending on the system).
    /// For example, `0o040000` identifies a directory, while `0o100000` identifies a regular file.
    ///
    /// To extract the permissions for each of the owner, group, and others:
    /// - The owner, group, and others' permissions are stored in **three groups of 3 bits** each (rwx).
    /// - Each permission group (e.g., owner, group, others) is shifted to the appropriate position in the `mode` value
    ///   and checked. For instance:
    ///   - `permissions(mode, 8)` checks the owner's **read** permission (the first bit).
    ///   - `permissions(mode, 7)` checks the owner's **write** permission (the second bit).
    ///   - `permissions(mode, 6)` checks the owner's **execute** permission (the third bit).
    /// - Similarly, we shift and check the corresponding permission bits for group and others.
    ///
    /// The shift is necessary because the `mode` value is a compact representation of the file's **type** and **permissions**,
    /// and shifting allows us to access each specific permission bit for the corresponding group (owner, group, others).
    fn format_permissions(&self, mode: u32) -> String {
        let file_type = match mode & FILE_TYPE_MASK {
            constants::DIRECTORY_TYPE_MASK => 'd',
            constants::REGULAR_FILE_TYPE_MASK => '-',
            constants::SYMLINK_TYPE_MASK => 'l',
            constants::BLOCK_DEVICE_TYPE_MASK => 'b',
            constants::CHARACTER_DEVICE_TYPE_MASK => 'c',
            constants::NAMED_PIPE_TYPE_MASK => 'p',
            constants::SOCKET_TYPE_MASK => 's',
            _ => '?',
        };

        let permissions =
            |mode: u32, shift: u32| -> char { if mode & (1 << shift) != 0 { 'r' } else { '-' } };

        format!(
            "{}{}{}{}{}{}{}{}{}{}",
            file_type,
            permissions(mode, 8),
            permissions(mode, 7),
            if mode & OWNER_PERMISSIONS_MASK != 0 {
                'x'
            } else {
                '-'
            },
            permissions(mode, 5),
            permissions(mode, 4),
            if mode & GROUP_PERMISSIONS_MASK != 0 {
                'x'
            } else {
                '-'
            },
            permissions(mode, 2),
            permissions(mode, 1),
            if mode & OTHERS_PERMISSIONS_MASK != 0 {
                'x'
            } else {
                '-'
            },
        )
    }

    pub fn create_writable_file(path: &str) -> std::io::Result<File> {
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
    }

    pub fn open_file_in_buffered_mode(file: &str) -> Option<BufReader<File>> {
        if let Ok(file) = File::open(file) {
            return Some(BufReader::new(file));
        }

        None
    }

    pub fn delete_file(&self, file: &str) -> bool {
        let file = self.get_relative_path(file);
        fs::remove_file(file).is_ok()
    }

    pub fn create_appendable_file(path: &str) -> std::io::Result<File> {
        fs::OpenOptions::new().create(true).append(true).open(path)
    }
}
