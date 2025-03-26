pub mod context;
pub mod handler;
pub mod types;

use crate::command::context::CommandContext;
use crate::command::types::CommandType;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct Command<'a> {
    command_type: Option<CommandType<'a>>,
}

impl<'a> Command<'a> {
    pub fn new(command_type: Option<CommandType<'a>>) -> Self {
        Self { command_type }
    }

    pub fn handle(&self, context: CommandContext) -> ResponseCollection {
        if self.command_type.is_some() {
            let command_type = self.command_type.as_ref().unwrap();
            let handler = command_type.get_handler();

            if command_type.has_a_missing_argument() {
                return self.missing_argument_response();
            }

            if command_type.requires_authentication() && !context.is_authenticated() {
                return self.authentication_required_response();
            }

            return handler.handle(context);
        }

        self.wrong_command_response()
    }

    fn missing_argument_response(&self) -> ResponseCollection {
        vec![Response::new(
            ResponseCode::SyntaxErrorInParametersOrArguments,
            ResponseMessage::MissingArgument,
            ResponseType::Complete,
        )]
    }

    fn authentication_required_response(&self) -> ResponseCollection {
        vec![Response::new(
            ResponseCode::NotLoggedIn,
            ResponseMessage::Custom("Please log in with USER and PASS first"),
            ResponseType::Complete,
        )]
    }

    fn wrong_command_response(&self) -> ResponseCollection {
        vec![Response::new(
            ResponseCode::SyntaxError,
            ResponseMessage::WrongCommand,
            ResponseType::Complete,
        )]
    }
}
