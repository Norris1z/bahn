use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct PasvHandler {}

impl CommandHandler for PasvHandler {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        if context.has_data_connection() {
            return vec![Response::new(
                ResponseCode::BadSequence,
                ResponseMessage::Custom("Connection already open"),
                ResponseType::Complete,
            )];
        }

        let address = context.create_passive_data_connection();

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
