use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseType};

pub mod user;

pub trait CommandHandler {
    fn command_can_be_executed(&self) -> bool {
        true
    }

    fn error(&self) -> Response {
        Response::new(
            ResponseCode::MissingArgument,
            ResponseMessage::MissingArgument,
            ResponseType::Complete,
        )
    }

    fn handle(&self) -> Response;
}
