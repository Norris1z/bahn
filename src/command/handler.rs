use crate::command::context::CommandContext;
use crate::response::ResponseCollection;

mod cdup;
mod cwd;
mod help;
mod list;
mod mkd;
mod nlst;
mod pass;
mod pasv;
mod pwd;
mod quit;
mod rmd;
mod rtype;
mod user;

pub use crate::command::handler::cdup::*;
pub use crate::command::handler::cwd::*;
pub use crate::command::handler::help::*;
pub use crate::command::handler::list::*;
pub use crate::command::handler::mkd::*;
pub use crate::command::handler::nlst::*;
pub use crate::command::handler::pass::*;
pub use crate::command::handler::pasv::*;
pub use crate::command::handler::pwd::*;
pub use crate::command::handler::quit::*;
pub use crate::command::handler::rmd::*;
pub use crate::command::handler::rtype::*;
pub use crate::command::handler::user::*;

pub trait CommandHandler {
    fn handle(&self, context: CommandContext) -> ResponseCollection;
}
