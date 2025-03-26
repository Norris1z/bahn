mod virtual_filesystem;
pub mod file;

pub mod filesystem {
    pub use crate::filesystem::virtual_filesystem::*;
}
