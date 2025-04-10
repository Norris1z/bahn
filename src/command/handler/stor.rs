use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::data::{DataTransferType, ResponseData, ResponseDataContentType};
use crate::response::file::FileResponse;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct StorCommandHandler<'a> {
    file: &'a CommandArgument<'a>,
}

impl<'a> StorCommandHandler<'a> {
    pub fn new(file: &'a CommandArgument<'a>) -> Self {
        Self { file }
    }
}

impl<'a> CommandHandler for StorCommandHandler<'a> {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        if !context.has_data_connection() {
            return vec![];
        }

        vec![
            Response::new(
                ResponseCode::StartingDataTransfer,
                ResponseMessage::StartingDataTransfer,
                ResponseType::Complete,
            ),
            Response::with_data(
                ResponseCode::Success,
                ResponseMessage::SendingDataToDataConnection,
                ResponseType::DataTransfer,
                ResponseData::new(
                    DataTransferType::Incoming,
                    ResponseDataContentType::File(FileResponse::new(
                        context.get_relative_path(self.file.as_ref().unwrap()),
                    )),
                ),
            ),
        ]
    }
}
