use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct PasvHandler {}

impl CommandHandler for PasvHandler {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        let address = context.create_data_connection();

        if address.is_none() {
            return vec![Response::new(
                ResponseCode::CantOpenDataConnection,
                ResponseMessage::CantOpenDataConnection,
                ResponseType::Complete,
            )];
        }

        vec![Response::new(
            ResponseCode::EnteringPassiveMode,
            ResponseMessage::CustomString(format!("Entering Passive Mode ({})", address.unwrap())),
            ResponseType::Complete,
        )]
    }
}
