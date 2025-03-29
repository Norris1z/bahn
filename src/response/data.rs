pub enum DataTransferType {
    Incoming,
    Outgoing,
}

impl PartialEq for DataTransferType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataTransferType::Incoming, DataTransferType::Incoming) => true,
            (DataTransferType::Outgoing, DataTransferType::Outgoing) => true,
            _ => false,
        }
    }
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
