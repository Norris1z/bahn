use crate::command::context::CommandContext;
use crate::response::ResponseCollection;

pub mod cdup;
pub mod cwd;
pub mod help;
pub mod mkd;
pub mod pass;
pub mod pwd;
pub mod quit;
pub mod user;
pub mod rtype;
pub mod pasv;
pub mod nlst;
pub mod list;

pub trait CommandHandler {
    fn handle(&self, context: CommandContext) -> ResponseCollection;
}
