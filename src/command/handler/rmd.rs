use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct RmdCommandHandler<'a> {
    path: &'a CommandArgument<'a>,
}

impl<'a> RmdCommandHandler<'a> {
    pub fn new(path: &'a CommandArgument<'a>) -> Self {
        Self { path }
    }
}

impl<'a> CommandHandler for RmdCommandHandler<'a> {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        let path = self.path.as_deref().unwrap();

        if !context.file_or_directory_exists(path) {
            return vec![Response::new(
                ResponseCode::RequestedActionNotTaken,
                ResponseMessage::Custom("Directory does not exist"),
                ResponseType::Complete,
            )];
        }

        if !context.delete_directory(path) {
            return vec![Response::new(
                ResponseCode::RequestedActionNotTaken,
                ResponseMessage::Custom("Failed to delete directory"),
                ResponseType::Complete,
            )];
        }

        vec![Response::new(
            ResponseCode::FileActionOkay,
            ResponseMessage::Custom("Directory deleted"),
            ResponseType::Complete,
        )]
    }
}
