use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct CdupCommandHandler {}

impl CommandHandler for CdupCommandHandler {
    fn requires_authentication(&self) -> bool {
        true
    }

    fn handle(&self, context: CommandContext) -> ResponseCollection {
        context.change_directory("..");

        vec![Response::new(
            ResponseCode::Success,
            ResponseMessage::Custom("Working directory changed to parent directory"),
            ResponseType::Complete,
        )]
    }
}
