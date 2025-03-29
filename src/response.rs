pub mod codes;
pub mod data;
pub mod messages;

use crate::response::codes::ResponseCode;
use crate::response::data::ResponseData;
use crate::response::messages::ResponseMessage;

pub type ResponseCollection = Vec<Response>;

pub enum ResponseType {
    Partial,
    Complete,
    Terminate,
    DataTransfer,
}

pub struct Response {
    code: ResponseCode,
    pub message: ResponseMessage,
    response_type: ResponseType,
    data: Option<ResponseData>,
}

impl Response {
    pub fn new(code: ResponseCode, message: ResponseMessage, response_type: ResponseType) -> Self {
        Self {
            code,
            message,
            response_type,
            data: None,
        }
    }

    pub fn is_terminate(&self) -> bool {
        matches!(self.response_type, ResponseType::Terminate)
    }

    pub fn is_partial(&self) -> bool {
        matches!(self.response_type, ResponseType::Partial)
    }

    pub fn set_data(&mut self, data: ResponseData) {
        self.data = Some(data);
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}\r\n",
            self.code as u16,
            if self.is_partial() { "-" } else { " " },
            self.message.get_message()
        )
    }
}
