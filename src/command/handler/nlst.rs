use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
use std::borrow::Cow;

pub struct NlstCommandHandler<'a> {
    path: &'a CommandArgument<'a>,
}

impl<'a> NlstCommandHandler<'a> {
    pub fn new(path: &'a CommandArgument<'a>) -> Self {
        Self { path }
    }
}

impl<'a> CommandHandler for NlstCommandHandler<'a> {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        if !context.has_data_connection() {
            return vec![];
        }

        let content = context.list_directory_content_names(
            self.path.as_ref().unwrap_or_else(|| &Cow::Borrowed(".")),
        );

        vec![Response::new(
            ResponseCode::Success,
            ResponseMessage::CustomString(content.join("\n").into()), //TODO: handle new lines properly
            ResponseType::Complete,
        )]
    }
}
