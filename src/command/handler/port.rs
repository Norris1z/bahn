use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct PortCommandHandler<'a> {
    address: &'a CommandArgument<'a>,
}

impl<'a> PortCommandHandler<'a> {
    pub fn new(address: &'a CommandArgument<'a>) -> Self {
        Self { address }
    }
}

impl<'a> CommandHandler for PortCommandHandler<'a> {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        if context.has_data_connection() {
            return vec![Response::new(
                ResponseCode::BadSequence,
                ResponseMessage::Custom("Connection already open"),
                ResponseType::Complete,
            )];
        }

        let address = context.construct_socket_addr(self.address.as_ref().unwrap());

        if address.is_none() {
            return vec![Response::new(
                ResponseCode::SyntaxErrorInParametersOrArguments,
                ResponseMessage::Custom("Invalid address"),
                ResponseType::Complete,
            )];
        }

        if !context.create_active_data_connection(address.unwrap()) {
            return vec![Response::new(
                ResponseCode::CantOpenDataConnection,
                ResponseMessage::CantOpenDataConnection,
                ResponseType::Complete,
            )];
        }

        vec![Response::new(
            ResponseCode::Success,
            ResponseMessage::Custom("PORT command successful"),
            ResponseType::Complete,
        )]
    }
}
