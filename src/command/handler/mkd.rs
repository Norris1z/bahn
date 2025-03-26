use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct MkdCommandHandler<'a> {
    path: &'a CommandArgument<'a>,
}

impl<'a> MkdCommandHandler<'a> {
    pub fn new(path: &'a CommandArgument<'a>) -> Self {
        Self { path }
    }
}
impl<'a> CommandHandler for MkdCommandHandler<'a> {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        let path = self.path.as_deref().unwrap();

        if context.directory_exists(path) {
            return vec![Response::new(
                ResponseCode::RequestedActionNotTaken,
                ResponseMessage::Custom("File or directory already exists"),
                ResponseType::Complete,
            )];
        }

        let directory = context.create_directory(path);

        if directory.is_none() {
            return vec![Response::new(
                ResponseCode::RequestedActionNotTaken,
                ResponseMessage::Custom("Unknown error"),
                ResponseType::Complete,
            )];
        }

        vec![Response::new(
            ResponseCode::DirectoryName,
            ResponseMessage::DirectoryNameCommentary(directory.unwrap(), "created successfully"),
            ResponseType::Complete,
        )]
    }
}
