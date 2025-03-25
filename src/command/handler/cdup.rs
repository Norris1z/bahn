use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
use crate::session::user::User;
use std::cell::RefCell;

pub struct CdupCommandHandler<'a> {
    user: &'a RefCell<User>,
}

impl<'a> CdupCommandHandler<'a> {
    pub fn new(user: &'a RefCell<User>) -> Self {
        Self { user }
    }
}

impl<'a> CommandHandler for CdupCommandHandler<'a> {
    fn requires_authentication(&self) -> bool {
        true
    }

    fn handle(&self) -> ResponseCollection {
        let user = self.user.borrow();

        let mut filesystem = user.filesystem.as_ref().unwrap().borrow_mut();

        filesystem.change_directory("..");

        vec![Response::new(
            ResponseCode::Success,
            ResponseMessage::Custom("Working directory changed to parent directory"),
            ResponseType::Complete,
        )]
    }
}
