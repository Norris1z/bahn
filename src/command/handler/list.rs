use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::data::{DataTransferType, ResponseData, ResponseDataContentType};
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
use std::borrow::Cow;

pub struct ListCommandHandler<'a> {
    path: &'a CommandArgument<'a>,
}

impl<'a> ListCommandHandler<'a> {
    pub fn new(path: &'a CommandArgument<'a>) -> Self {
        Self { path }
    }
}

impl<'a> CommandHandler for ListCommandHandler<'a> {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        if !context.has_data_connection() {
            return vec![];
        }

        let content = context.list_directory_detailed_content_information(
            self.path.as_ref().unwrap_or_else(|| &Cow::Borrowed(".")),
        );

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
                    DataTransferType::Outgoing,
                    ResponseDataContentType::FileInfoList(content),
                ),
            ),
        ]
    }
}
