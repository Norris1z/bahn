pub struct FileResponse {
    pub filename: String,
}

impl FileResponse {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }
}
