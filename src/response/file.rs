#[derive(PartialEq)]
pub enum FileMode {
    Create,
    Append,
}

pub struct FileResponse {
    pub filename: String,
    pub mode: Option<FileMode>,
}

impl FileResponse {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            mode: None,
        }
    }

    pub fn with_mode(filename: String, mode: FileMode) -> Self {
        Self {
            filename,
            mode: Some(mode),
        }
    }
}
