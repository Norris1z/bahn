use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct QuitCommandHandler {}

impl CommandHandler for QuitCommandHandler {
    fn handle(&self, _: CommandContext) -> ResponseCollection {
        vec![Response::new(
            ResponseCode::Quit,
            ResponseMessage::Quit,
            ResponseType::Terminate,
        )]
    }
}
