use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct HelpCommandHandler {}

impl CommandHandler for HelpCommandHandler {
    fn handle(&self) -> ResponseCollection {
        vec![
            Response::new(
                ResponseCode::Help,
                ResponseMessage::Custom("The following commands are supported. \r\n USER HELP"),
                ResponseType::Partial,
            ),
            Response::new(
                ResponseCode::Help,
                ResponseMessage::Help,
                ResponseType::Complete,
            ),
        ]
    }
}
