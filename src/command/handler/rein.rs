use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct ReinCommandHandler {}

impl CommandHandler for ReinCommandHandler {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        context.reinitialize_user_state();

        vec![Response::new(
            ResponseCode::ServiceReadyForNewUser,
            ResponseMessage::ServiceReadyForNewUser,
            ResponseType::Complete,
        )]
    }
}
