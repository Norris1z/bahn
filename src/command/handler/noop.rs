use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct NoopCommandHandler {}

impl CommandHandler for NoopCommandHandler {
    fn handle(&self, _: CommandContext) -> ResponseCollection {
        vec![Response::new(
            ResponseCode::Success,
            ResponseMessage::NoopOkay,
            ResponseType::Complete,
        )]
    }
}
