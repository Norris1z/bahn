use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
use crate::session::user::User;
use std::cell::RefCell;

pub struct CwdCommandHandler<'a> {
    path: &'a CommandArgument<'a>,
    user: &'a RefCell<User>,
}

impl<'a> CwdCommandHandler<'a> {
    pub fn new(path: &'a CommandArgument<'a>, user: &'a RefCell<User>) -> Self {
        Self { path, user }
    }
}

impl<'a> CommandHandler for CwdCommandHandler<'a> {
    fn requires_authentication(&self) -> bool {
        true
    }

    fn command_can_be_executed(&self) -> bool {
        self.path.is_some()
    }

    fn handle(&self) -> ResponseCollection {
        let user = self.user.borrow();

        let mut filesystem = user.filesystem.as_ref().unwrap().borrow_mut();

        let path = self.path.as_deref().unwrap();

        if !filesystem.exists(path) {
            return vec![Response::new(
                ResponseCode::RequestedActionNotTaken,
                ResponseMessage::Custom("Directory does not exist"),
                ResponseType::Complete,
            )];
        }

        filesystem.change_directory(path);

        vec![Response::new(
            ResponseCode::FileActionOkay,
            ResponseMessage::Custom("Working directory changed"),
            ResponseType::Complete,
        )]
    }
}
