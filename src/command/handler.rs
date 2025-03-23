use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub mod help;
pub mod pass;
pub mod quit;
pub mod user;
pub mod pwd;

pub trait CommandHandler {
    fn requires_authentication(&self) -> bool {
        true
    }

    fn command_can_be_executed(&self) -> bool {
        true
    }

    fn error(&self) -> ResponseCollection {
        vec![Response::new(
            ResponseCode::SyntaxErrorInParametersOrArguments,
            ResponseMessage::MissingArgument,
            ResponseType::Complete,
        )]
    }

    fn handle(&self) -> ResponseCollection;
}
