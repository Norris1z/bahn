use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct DeleCommandHandler<'a> {
    file: &'a CommandArgument<'a>,
}

impl<'a> DeleCommandHandler<'a> {
    pub fn new(file: &'a CommandArgument<'a>) -> Self {
        Self { file }
    }
}
impl<'a> CommandHandler for DeleCommandHandler<'a> {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        if !context.file_exists(self.file.as_ref().unwrap()) {
            return vec![Response::new(
                ResponseCode::RequestedActionNotTaken,
                ResponseMessage::Custom("File does not exist"),
                ResponseType::Complete,
            )];
        }

        if !context.delete_file(self.file.as_ref().unwrap()) {
            return vec![Response::new(
                ResponseCode::RequestedActionNotTaken,
                ResponseMessage::Custom("Failed to delete file"),
                ResponseType::Complete,
            )];
        }

        vec![Response::new(
            ResponseCode::FileActionOkay,
            ResponseMessage::Custom("File deleted successfully"),
            ResponseType::Complete,
        )]
    }
}
