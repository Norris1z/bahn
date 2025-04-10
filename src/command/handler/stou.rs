use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::data::{DataTransferType, ResponseData, ResponseDataContentType};
use crate::response::file::FileResponse;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct StouCommandHandler {}

impl CommandHandler for StouCommandHandler {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        let file = context.random_filename();
        let path = context.get_relative_path(file.as_str());

        vec![
            Response::new(
                ResponseCode::StartingDataTransfer,
                ResponseMessage::CustomString(format!("FILE: {}", file)),
                ResponseType::Complete,
            ),
            Response::with_data(
                ResponseCode::Success,
                ResponseMessage::SendingDataToDataConnection,
                ResponseType::DataTransfer,
                ResponseData::new(
                    DataTransferType::Incoming,
                    ResponseDataContentType::File(FileResponse::new(path)),
                ),
            ),
        ]
    }
}
