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
    fn requires_authentication(&self) -> bool {
        true
    }

    fn handle(&self) -> ResponseCollection {
        let user = self.user.borrow();

        let filesystem = user.filesystem.as_ref();

        vec![Response::new(
            ResponseCode::DirectoryName,
            ResponseMessage::DirectoryNameCommentary(
                filesystem.unwrap().borrow().get_current_directory(),
                "is the current directory",
            ),
            ResponseType::Complete,
        )]
    }
}
