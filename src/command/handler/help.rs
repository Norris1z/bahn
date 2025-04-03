use crate::command::context::CommandContext;
use crate::command::handler::CommandHandler;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};

pub struct HelpCommandHandler {}

impl CommandHandler for HelpCommandHandler {
    fn handle(&self, _: CommandContext) -> ResponseCollection {
        vec![
            Response::new(
                ResponseCode::Help,
                ResponseMessage::Custom(
                    r#"The following commands are supported.
 USER HELP PASS QUIT PWD MKD CWD CDUP LIST NLST
 PASV PORT RMD RETR STOR TYPE NOOP SYST DELE"#,
                ),
                ResponseType::Partial,
            ),
            Response::new(
                ResponseCode::Help,
                ResponseMessage::Help,
                ResponseType::Complete,
            ),
        ]
    }
}
