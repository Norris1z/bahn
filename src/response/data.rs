use crate::response::file::FileResponse;

pub enum DataTransferType {
    Incoming,
    Outgoing,
}

pub struct ResponseData {
    pub transfer_type: DataTransferType,
    pub content: ResponseDataContentType,
}

pub enum ResponseDataContentType {
    FileInfoList(Vec<String>),
    File(FileResponse),
}

impl ResponseData {
    pub fn new(transfer_type: DataTransferType, content: ResponseDataContentType) -> Self {
        Self {
            transfer_type,
            content,
        }
    }
}
