use crate::auth::Auth;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
use crate::session::user::User;
use std::cell::RefCell;

pub struct PassCommandHandler<'a> {
    password: &'a CommandArgument<'a>,
    user: &'a RefCell<User>,
}

impl<'a> PassCommandHandler<'a> {
    pub fn new(password: &'a CommandArgument<'a>, user: &'a RefCell<User>) -> Self {
        Self { password, user }
    }
}

impl<'a> CommandHandler for PassCommandHandler<'a> {
    fn command_can_be_executed(&self) -> bool {
        self.password.is_some()
    }

    fn handle(&self) -> ResponseCollection {
        if self.user.borrow().is_authenticated {
            return vec![Response::new(
                ResponseCode::BadSequence,
                ResponseMessage::Custom("Already authenticated."),
                ResponseType::Complete,
            )];
        }

        if self.user.borrow().username.is_none() {
            return vec![Response::new(
                ResponseCode::BadSequence,
                ResponseMessage::Custom("Login with USER first"),
                ResponseType::Complete,
            )];
        }

        let path = Auth::attempt(
            self.user.borrow().username.as_deref().unwrap(),
            self.password.as_deref().unwrap(),
        );

        if path.is_none() {
            return vec![Response::new(
                ResponseCode::NotLoggedIn,
                ResponseMessage::Custom("Login Incorrect"),
                ResponseType::Complete,
            )];
        }

        self.user.borrow_mut().is_authenticated = true;
        self.user.borrow_mut().path = path;

        vec![Response::new(
            ResponseCode::LoginSuccessful,
            ResponseMessage::LoginSuccessful,
            ResponseType::Complete,
        )]
    }
}
