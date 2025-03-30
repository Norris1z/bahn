use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::data::{DataTransferType, ResponseData};
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

        let mut response = Response::new(
            ResponseCode::Success,
            ResponseMessage::SendingDataToDataConnection,
            ResponseType::DataTransfer,
        );

        response.set_data(ResponseData::new(
            DataTransferType::Incoming,
            vec![context.get_relative_path(self.file.as_ref().unwrap())],
        ));

        vec![response]
    }
}
