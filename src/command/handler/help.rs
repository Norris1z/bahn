use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct HelpCommandHandler {}

impl CommandHandler for HelpCommandHandler {
    fn requires_authentication(&self) -> bool {
        false
    }

    fn handle(&self) -> ResponseCollection {
        vec![
            Response::new(
                ResponseCode::Help,
                ResponseMessage::Custom("The following commands are supported. \r\n USER HELP PASS QUIT"),
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
