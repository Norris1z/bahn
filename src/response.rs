pub mod codes;
pub mod messages;

use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;

pub type ResponseCollection = Vec<Response>;

pub enum ResponseType {
    Partial,
    Complete,
    Terminate,
}

pub struct Response {
    code: ResponseCode,
    pub message: ResponseMessage,
    response_type: ResponseType,
}

impl Response {
    pub fn new(code: ResponseCode, message: ResponseMessage, response_type: ResponseType) -> Self {
        Self {
            code,
            message,
            response_type,
        }
    }

    pub fn is_terminate(&self) -> bool {
        matches!(self.response_type, ResponseType::Terminate)
    }

    pub fn is_partial(&self) -> bool {
        matches!(self.response_type, ResponseType::Partial)
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
