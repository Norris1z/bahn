use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct UserCommandHandler<'a> {
    name: &'a CommandArgument<'a>,
}

impl<'a> UserCommandHandler<'a> {
    pub fn new(name: &'a CommandArgument<'a>) -> Self {
        Self { name }
    }
}

impl<'a> CommandHandler for UserCommandHandler<'a> {
    fn requires_authentication(&self) -> bool {
        false
    }

    fn command_can_be_executed(&self) -> bool {
        self.name.is_some()
    }

    fn handle(&self, context: CommandContext) -> ResponseCollection {
        if context.is_authenticated() {
            return vec![Response::new(
                ResponseCode::BadSequence,
                ResponseMessage::Custom("Already authenticated. QUIT first"),
                ResponseType::Complete,
            )];
        }

        context.set_username(self.name.as_deref().unwrap().to_string());

        vec![Response::new(
            ResponseCode::UserNameOkay,
            ResponseMessage::UserNameOkay,
            ResponseType::Complete,
        )]
    }
}
