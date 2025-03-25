use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
use crate::session::user::User;
use std::cell::RefCell;

pub struct MkdCommandHandler<'a> {
    path: &'a CommandArgument<'a>,
    user: &'a RefCell<User>,
}

impl<'a> MkdCommandHandler<'a> {
    pub fn new(path: &'a CommandArgument<'a>, user: &'a RefCell<User>) -> Self {
        Self { path, user }
    }
}
impl<'a> CommandHandler for MkdCommandHandler<'a> {
    fn requires_authentication(&self) -> bool {
        true
    }

    fn command_can_be_executed(&self) -> bool {
        self.path.is_some()
    }

    fn handle(&self) -> ResponseCollection {
        let user_borrow = self.user.borrow();

        let filesystem = user_borrow.filesystem.as_ref().unwrap();

        let path = self.path.as_deref().unwrap();

        if filesystem.borrow().exists(path) {
            return vec![Response::new(
                ResponseCode::RequestedActionNotTaken,
                ResponseMessage::Custom("File or directory already exists"),
                ResponseType::Complete,
            )];
        }

        let directory = filesystem.borrow().create_directory(path);

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
