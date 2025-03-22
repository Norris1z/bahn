use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct QuitCommandHandler {}

impl CommandHandler for QuitCommandHandler {
    fn requires_authentication(&self) -> bool {
        false
    }

    fn handle(&self) -> ResponseCollection {
        vec![Response::new(
            ResponseCode::Quit,
            ResponseMessage::Quit,
            ResponseType::Terminate,
        )]
    }
}
