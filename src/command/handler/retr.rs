use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::data::{DataTransferType, ResponseData, ResponseDataContentType};
use crate::response::file::FileResponse;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct RetrCommandHandler<'a> {
    file: &'a CommandArgument<'a>,
}

impl<'a> RetrCommandHandler<'a> {
    pub fn new(file: &'a CommandArgument<'a>) -> Self {
        Self { file }
    }
}

impl<'a> CommandHandler for RetrCommandHandler<'a> {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        let file = self.file.as_deref().unwrap();

        if !context.file_or_directory_exists(file) {
            return vec![Response::new(
                ResponseCode::RequestedActionNotTaken,
                ResponseMessage::Custom("File does not exist"),
                ResponseType::Complete,
            )];
        }

        vec![Response::with_data(
            ResponseCode::Success,
            ResponseMessage::SendingDataToDataConnection,
            ResponseType::DataTransfer,
            ResponseData::new(
                DataTransferType::Outgoing,
                ResponseDataContentType::File(FileResponse::new(
                    context.get_relative_path(self.file.as_ref().unwrap()),
                )),
            ),
        )]
    }
}
