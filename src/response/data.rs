pub enum DataTransferType {
    Incoming,
    Outgoing,
}

pub struct ResponseData {
    pub transfer_type: DataTransferType,
    pub content: Vec<String>,
}

impl ResponseData {
    pub fn new(transfer_type: DataTransferType, content: Vec<String>) -> Self {
        Self {
            transfer_type,
            content,
        }
    }
}
