pub mod context;
pub mod handler;
pub mod types;

use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::command::handler::cdup::CdupCommandHandler;
use crate::command::handler::cwd::CwdCommandHandler;
use crate::command::handler::help::HelpCommandHandler;
use crate::command::handler::mkd::MkdCommandHandler;
use crate::command::handler::pass::PassCommandHandler;
use crate::command::handler::pwd::PwdCommandHandler;
use crate::command::handler::quit::QuitCommandHandler;
use crate::command::handler::user::UserCommandHandler;
use crate::command::types::CommandType;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
use crate::session::user::User;
use std::cell::RefCell;

pub struct Command<'a> {
    command_type: Option<CommandType<'a>>,
    user: &'a RefCell<User>,
}

impl<'a> Command<'a> {
    pub fn new(command_type: Option<CommandType<'a>>, user: &'a RefCell<User>) -> Self {
        Self { command_type, user }
    }

    pub fn handle(&self, context: CommandContext) -> ResponseCollection {
        if self.command_type.is_some() {
            let possible_handler: Option<Box<dyn CommandHandler>> = match &self.command_type {
                Some(CommandType::User(name)) => Some(Box::new(UserCommandHandler::new(name))),
                Some(CommandType::Pass(password)) => {
                    Some(Box::new(PassCommandHandler::new(password)))
                }
                Some(CommandType::Help) => Some(Box::new(HelpCommandHandler {})),
                Some(CommandType::Quit) => Some(Box::new(QuitCommandHandler {})),
                Some(CommandType::Pwd) => Some(Box::new(PwdCommandHandler {})),
                Some(CommandType::Mkd(path)) => Some(Box::new(MkdCommandHandler::new(path))),
                Some(CommandType::Cwd(path)) => Some(Box::new(CwdCommandHandler::new(path))),
                Some(CommandType::Cdup) => Some(Box::new(CdupCommandHandler {})),
                _ => None,
            };

            if possible_handler.is_some() {
                let handler = possible_handler.unwrap();

                if handler.requires_authentication() && !self.user.borrow().is_authenticated {
                    return vec![Response::new(
                        ResponseCode::NotLoggedIn,
                        ResponseMessage::Custom("Please log in with USER and PASS first"),
                        ResponseType::Complete,
                    )];
                }

                if !handler.command_can_be_executed() {
                    return handler.error();
                }

                return handler.handle(context);
            }
        }

        vec![Response::new(
            ResponseCode::SyntaxError,
            ResponseMessage::WrongCommand,
            ResponseType::Complete,
        )]
    }
}
