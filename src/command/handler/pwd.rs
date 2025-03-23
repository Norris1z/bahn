use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
use crate::session::user::User;
use std::cell::RefCell;

pub struct PwdCommandHandler<'a> {
    user: &'a RefCell<User>,
}

impl<'a> PwdCommandHandler<'a> {
    pub fn new(user: &'a RefCell<User>) -> Self {
        Self { user }
    }
}

impl<'a> CommandHandler for PwdCommandHandler<'a> {
    fn handle(&self) -> ResponseCollection {
        let user_borrow = self.user.borrow();

        let filesystem = user_borrow.filesystem.as_ref();

        vec![Response::new(
            ResponseCode::DirectoryName,
            ResponseMessage::DirectoryNameCommentary(
                filesystem.unwrap().get_current_directory(),
                "is the current directory",
            ),
            ResponseType::Complete,
        )]
    }
}
