#[derive(Debug)]
pub enum DataTransferStatus {
    Success,
    Failed,
}

impl PartialEq for DataTransferStatus {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataTransferStatus::Success, DataTransferStatus::Success) => true,
            (DataTransferStatus::Failed, DataTransferStatus::Failed) => true,
            _ => false,
        }
    }
}
