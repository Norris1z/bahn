use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct StatCommandHandler<'a> {
    path: &'a CommandArgument<'a>,
}

impl<'a> StatCommandHandler<'a> {
    pub fn new(path: &'a CommandArgument<'a>) -> Self {
        Self { path }
    }

    fn directory_info(&self, context: &CommandContext) -> ResponseCollection {
        vec![
            Response::new(
                ResponseCode::DirectoryStatus,
                ResponseMessage::CustomString(format!(
                    "Status of {}:\n{}",
                    self.path.as_ref().unwrap(),
                    context
                        .list_directory_detailed_content_information(self.path.as_ref().unwrap())
                        .join("\n")
                )),
                ResponseType::Partial,
            ),
            Response::new(
                ResponseCode::DirectoryStatus,
                ResponseMessage::Custom("END"),
                ResponseType::Complete,
            ),
        ]
    }

    fn file_info(&self, context: &CommandContext) -> ResponseCollection {
        match context.file_metadata_information(self.path.as_ref().unwrap()) {
            Some(info) => vec![
                Response::new(
                    ResponseCode::FileStatus,
                    ResponseMessage::CustomString(format!(
                        "Status of {}:\n{}",
                        self.path.as_ref().unwrap(),
                        info
                    )),
                    ResponseType::Partial,
                ),
                Response::new(
                    ResponseCode::FileStatus,
                    ResponseMessage::Custom("END"),
                    ResponseType::Complete,
                ),
            ],
            None => vec![Response::new(
                ResponseCode::RequestedFileActionNotTaken,
                ResponseMessage::Custom("Unable to access file metadata"),
                ResponseType::Complete,
            )],
        }
    }
}

impl<'a> CommandHandler for StatCommandHandler<'a> {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        if self.path.is_some() {
            if context.is_file(self.path.as_ref().unwrap()) {
                return self.file_info(&context);
            }
            return self.directory_info(&context);
        }

        vec![
            Response::new(
                ResponseCode::SystemStatus,
                ResponseMessage::Custom("FTP Server status:\n Up and running, lovely!"),
                ResponseType::Partial,
            ),
            Response::new(
                ResponseCode::SystemStatus,
                ResponseMessage::Custom("End of status"),
                ResponseType::Complete,
            ),
        ]
    }
}
