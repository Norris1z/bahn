use crate::command::handler::{
    AppeCommandHandler, CdupCommandHandler, CommandHandler, CwdCommandHandler, DeleCommandHandler,
    HelpCommandHandler, ListCommandHandler, MkdCommandHandler, NlstCommandHandler,
    NoopCommandHandler, PassCommandHandler, PasvHandler, PortCommandHandler, PwdCommandHandler,
    QuitCommandHandler, ReinCommandHandler, RetrCommandHandler, RmdCommandHandler,
    StatCommandHandler, StorCommandHandler, StouCommandHandler, SystCommandHandler,
    TypeCommandHandler, UserCommandHandler,
};
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
    Pasv,
    Nlst(CommandArgument<'a>),
    List(CommandArgument<'a>),
    Rmd(CommandArgument<'a>),
    Stor(CommandArgument<'a>),
    Retr(CommandArgument<'a>),
    Port(CommandArgument<'a>),
    Noop,
    Syst,
    Dele(CommandArgument<'a>),
    Rein,
    Stou,
    Appe(CommandArgument<'a>),
    Stat(CommandArgument<'a>),
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
            _ if command.eq_ignore_ascii_case("pasv") => Some(CommandType::Pasv),
            _ if command.eq_ignore_ascii_case("nlst") => Some(CommandType::Nlst(
                command_iterator.next().map(Cow::Borrowed),
            )),
            _ if command.eq_ignore_ascii_case("list") => Some(CommandType::List(
                command_iterator.next().map(Cow::Borrowed),
            )),
            _ if command.eq_ignore_ascii_case("rmd") => {
                Some(CommandType::Rmd(command_iterator.next().map(Cow::Borrowed)))
            }
            _ if command.eq_ignore_ascii_case("stor") => Some(CommandType::Stor(
                command_iterator.next().map(Cow::Borrowed),
            )),
            _ if command.eq_ignore_ascii_case("retr") => Some(CommandType::Retr(
                command_iterator.next().map(Cow::Borrowed),
            )),
            _ if command.eq_ignore_ascii_case("port") => Some(CommandType::Port(
                command_iterator.next().map(Cow::Borrowed),
            )),
            _ if command.eq_ignore_ascii_case("noop") => Some(CommandType::Noop),
            _ if command.eq_ignore_ascii_case("syst") => Some(CommandType::Syst),
            _ if command.eq_ignore_ascii_case("dele") => Some(CommandType::Dele(
                command_iterator.next().map(Cow::Borrowed),
            )),
            _ if command.eq_ignore_ascii_case("rein") => Some(CommandType::Rein),
            _ if command.eq_ignore_ascii_case("stou") => Some(CommandType::Stou),
            _ if command.eq_ignore_ascii_case("appe") => Some(CommandType::Appe(
                command_iterator.next().map(Cow::Borrowed),
            )),
            _ if command.eq_ignore_ascii_case("stat") => Some(CommandType::Stat(
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
            | CommandType::Type(argument, _)
            | CommandType::Rmd(argument)
            | CommandType::Stor(argument)
            | CommandType::Retr(argument)
            | CommandType::Port(argument)
            | CommandType::Dele(argument)
            | CommandType::Appe(argument) => argument.is_none(),
            _ => false,
        }
    }

    pub fn requires_authentication(&self) -> bool {
        match self {
            CommandType::Help
            | CommandType::Quit
            | CommandType::User(_)
            | CommandType::Pass(_)
            | CommandType::Noop
            | CommandType::Syst => false,
            _ => true,
        }
    }

    pub fn should_send_via_data_connection(&self) -> bool {
        match self {
            CommandType::Nlst(_)
            | CommandType::List(_)
            | CommandType::Stor(_)
            | CommandType::Stou
            | CommandType::Retr(_)
            | CommandType::Appe(_) => true,
            _ => false,
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
            CommandType::Type(code, option) => Box::new(TypeCommandHandler::new(code, option)),
            CommandType::Pasv => Box::new(PasvHandler {}),
            CommandType::Nlst(path) => Box::new(NlstCommandHandler::new(path)),
            CommandType::List(path) => Box::new(ListCommandHandler::new(path)),
            CommandType::Rmd(path) => Box::new(RmdCommandHandler::new(path)),
            CommandType::Stor(file) => Box::new(StorCommandHandler::new(file)),
            CommandType::Retr(file) => Box::new(RetrCommandHandler::new(file)),
            CommandType::Port(address) => Box::new(PortCommandHandler::new(address)),
            CommandType::Noop => Box::new(NoopCommandHandler {}),
            CommandType::Syst => Box::new(SystCommandHandler {}),
            CommandType::Dele(file) => Box::new(DeleCommandHandler::new(file)),
            CommandType::Rein => Box::new(ReinCommandHandler {}),
            CommandType::Stou => Box::new(StouCommandHandler {}),
            CommandType::Appe(file) => Box::new(AppeCommandHandler::new(file)),
            CommandType::Stat(path) => Box::new(StatCommandHandler::new(path)),
        }
    }
}
