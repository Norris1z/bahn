pub mod codes;
pub mod messages;

use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;

pub type ResponseCollection = Vec<Response>;

pub enum ResponseType {
    Partial,
    Complete,
}

pub struct Response {
    code: ResponseCode,
    message: ResponseMessage,
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
}

impl ToString for Response {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}\r\n",
            self.code as u16,
            if matches!(self.response_type, ResponseType::Complete) {
                " "
            } else {
                "-"
            },
            self.message.get_message()
        )
    }
}
