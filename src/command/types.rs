use crate::command::handler::CommandHandler;
use crate::command::handler::cdup::CdupCommandHandler;
use crate::command::handler::cwd::CwdCommandHandler;
use crate::command::handler::help::HelpCommandHandler;
use crate::command::handler::mkd::MkdCommandHandler;
use crate::command::handler::pass::PassCommandHandler;
use crate::command::handler::pwd::PwdCommandHandler;
use crate::command::handler::quit::QuitCommandHandler;
use crate::command::handler::rtype::TypeHandler;
use crate::command::handler::user::UserCommandHandler;
use std::borrow::Cow;

pub type CommandArgument<'a> = Option<Cow<'a, str>>;
#[derive(Debug)]
pub enum CommandType<'a> {
    Help,
    Quit,
    User(CommandArgument<'a>),
    Pass(CommandArgument<'a>),
    Pwd,
    Mkd(CommandArgument<'a>),
    Cwd(CommandArgument<'a>),
    Cdup,
    Type(CommandArgument<'a>, CommandArgument<'a>),
}
impl<'a> CommandType<'a> {
    pub fn from(string: &'a str) -> Option<Self> {
        let mut command_iterator = string.split_whitespace();

        let command = command_iterator.next()?;

        match command {
            _ if command.eq_ignore_ascii_case("help") => Some(CommandType::Help),
            _ if command.eq_ignore_ascii_case("user") => Some(CommandType::User(
                command_iterator.next().map(Cow::Borrowed),
            )),
            _ if command.eq_ignore_ascii_case("quit") => Some(CommandType::Quit),
            _ if command.eq_ignore_ascii_case("pass") => Some(CommandType::Pass(
                command_iterator.next().map(Cow::Borrowed),
            )),
            _ if command.eq_ignore_ascii_case("pwd") => Some(CommandType::Pwd),
            _ if command.eq_ignore_ascii_case("mkd") => {
                Some(CommandType::Mkd(command_iterator.next().map(Cow::Borrowed)))
            }
            _ if command.eq_ignore_ascii_case("cwd") => {
                Some(CommandType::Cwd(command_iterator.next().map(Cow::Borrowed)))
            }
            _ if command.eq_ignore_ascii_case("cdup") => Some(CommandType::Cdup),
            _ if command.eq_ignore_ascii_case("type") => Some(CommandType::Type(
                command_iterator.next().map(Cow::Borrowed),
                command_iterator.next().map(Cow::Borrowed),
            )),
            _ => None,
        }
    }

    pub fn has_a_missing_argument(&self) -> bool {
        match self {
            CommandType::User(argument)
            | CommandType::Pass(argument)
            | CommandType::Mkd(argument)
            | CommandType::Cwd(argument)
            | CommandType::Type(argument, _) => argument.is_none(),
            _ => false,
        }
    }

    pub fn requires_authentication(&self) -> bool {
        match self {
            CommandType::Help | CommandType::Quit | CommandType::User(_) | CommandType::Pass(_) => {
                false
            }
            _ => true,
        }
    }

    pub fn get_handler(&self) -> Box<dyn CommandHandler + '_> {
        match self {
            CommandType::User(name) => Box::new(UserCommandHandler::new(name)),
            CommandType::Help => Box::new(HelpCommandHandler {}),
            CommandType::Quit => Box::new(QuitCommandHandler {}),
            CommandType::Pass(password) => Box::new(PassCommandHandler::new(password)),
            CommandType::Pwd => Box::new(PwdCommandHandler {}),
            CommandType::Mkd(path) => Box::new(MkdCommandHandler::new(path)),
            CommandType::Cwd(path) => Box::new(CwdCommandHandler::new(path)),
            CommandType::Cdup => Box::new(CdupCommandHandler {}),
            CommandType::Type(code, option) => Box::new(TypeHandler::new(code, option)),
        }
    }
}
