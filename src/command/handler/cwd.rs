use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct CwdCommandHandler<'a> {
    path: &'a CommandArgument<'a>,
}

impl<'a> CwdCommandHandler<'a> {
    pub fn new(path: &'a CommandArgument<'a>) -> Self {
        Self { path }
    }
}

impl<'a> CommandHandler for CwdCommandHandler<'a> {
    fn requires_authentication(&self) -> bool {
        true
    }

    fn command_can_be_executed(&self) -> bool {
        self.path.is_some()
    }

    fn handle(&self, context: CommandContext) -> ResponseCollection {
        let path = self.path.as_deref().unwrap();

        if !context.directory_exists(path) {
            return vec![Response::new(
                ResponseCode::RequestedActionNotTaken,
                ResponseMessage::Custom("Directory does not exist"),
                ResponseType::Complete,
            )];
        }

        context.change_directory(path);

        vec![Response::new(
            ResponseCode::FileActionOkay,
            ResponseMessage::Custom("Working directory changed"),
            ResponseType::Complete,
        )]
    }
}
