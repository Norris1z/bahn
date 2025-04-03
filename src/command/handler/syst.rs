use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
use std::env::consts::OS;

pub struct SystCommandHandler {}

impl CommandHandler for SystCommandHandler {
    fn handle(&self, _: CommandContext) -> ResponseCollection {
        vec![Response::new(
            ResponseCode::SystemType,
            ResponseMessage::CustomString(format!("{} system type", OS)),
            ResponseType::Complete,
        )]
    }
}
