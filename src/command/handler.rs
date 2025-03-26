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

pub trait CommandHandler {
    fn handle(&self, context: CommandContext) -> ResponseCollection;
}
