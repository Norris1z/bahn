use std::borrow::Cow;

pub type CommandArgument<'a> = Option<Cow<'a, str>>;
#[derive(Debug)]
pub enum CommandType<'a> {
    Help,
    User(CommandArgument<'a>),
    // Pass(CommandArgument<'a>),
    // Acct(CommandArgument<'a>),
    // Cwd(CommandArgument<'a>),
    // Cdup,
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
            // _ if command.eq_ignore_ascii_case("pass") => Some(CommandType::Pass(
            //     command_iterator.next().map(Cow::Borrowed),
            // )),
            // _ if command.eq_ignore_ascii_case("acct") => Some(CommandType::Acct(
            //     command_iterator.next().map(Cow::Borrowed),
            // )),
            // _ if command.eq_ignore_ascii_case("cwd") => {
            //     Some(CommandType::Cwd(command_iterator.next().map(Cow::Borrowed)))
            // }
            // _ if command.eq_ignore_ascii_case("cdup") => Some(CommandType::Cdup),
            _ => None,
        }
    }
}
