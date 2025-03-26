use crate::auth::Auth;
use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::types::CommandArgument;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct PassCommandHandler<'a> {
    password: &'a CommandArgument<'a>,
}

impl<'a> PassCommandHandler<'a> {
    pub fn new(password: &'a CommandArgument<'a>) -> Self {
        Self { password }
    }
}

impl<'a> CommandHandler for PassCommandHandler<'a> {
    fn handle(&self, context: CommandContext) -> ResponseCollection {
        if context.is_authenticated() {
            return vec![Response::new(
                ResponseCode::BadSequence,
                ResponseMessage::Custom("Already authenticated."),
                ResponseType::Complete,
            )];
        }

        let username = context.get_username();

        if username.is_none() {
            return vec![Response::new(
                ResponseCode::BadSequence,
                ResponseMessage::Custom("Login with USER first"),
                ResponseType::Complete,
            )];
        }

        let path = Auth::attempt(
            username.unwrap().as_str(),
            self.password.as_deref().unwrap(),
        );

        if path.is_none() {
            return vec![Response::new(
                ResponseCode::NotLoggedIn,
                ResponseMessage::Custom("Login Incorrect"),
                ResponseType::Complete,
            )];
        }

        context.initialize_user_environment(path);

        vec![Response::new(
            ResponseCode::LoginSuccessful,
            ResponseMessage::LoginSuccessful,
            ResponseType::Complete,
        )]
    }
}
