use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct PwdCommandHandler {}

impl CommandHandler for PwdCommandHandler {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        vec![Response::new(
            ResponseCode::DirectoryName,
            ResponseMessage::DirectoryNameCommentary(
                context.get_current_directory(),
                "is the current directory",
            ),
            ResponseType::Complete,
        )]
    }
}
