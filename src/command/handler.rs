use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub mod cdup;
pub mod cwd;
pub mod help;
pub mod mkd;
pub mod pass;
pub mod pwd;
pub mod quit;
pub mod user;

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
