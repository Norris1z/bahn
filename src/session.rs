use crate::command::Command;
use crate::command::context::CommandContext;
use crate::command::types::CommandType;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
use std::cell::RefCell;
use tokio::net::tcp::OwnedWriteHalf;
use user::User;

pub mod user;

pub struct Session {
    user: RefCell<User>,
    socket_writer: OwnedWriteHalf,
}

impl Session {
    pub fn new(socket_writer: OwnedWriteHalf) -> Self {
        Self {
            user: RefCell::new(User::new()),
            socket_writer,
        }
    }

    pub fn process(&mut self, command_type: Option<CommandType>) -> bool {
        let command = Command::new(command_type, &self.user);
        let context = CommandContext::new(&self.user);

        self.send_response(command.handle(context))
    }

    fn send_response(&mut self, responses: ResponseCollection) -> bool {
        responses.iter().for_each(|response| {
            self.socket_writer
                .try_write(response.to_string().as_bytes())
                .unwrap();
        });

        let last_response = responses.last().unwrap();

        !last_response.is_terminate()
    }

    pub fn init(&mut self) {
        self.send_response(vec![
            Response::new(
                ResponseCode::Success,
                ResponseMessage::Greeting,
                ResponseType::Partial,
            ),
            Response::new(
                ResponseCode::Success,
                ResponseMessage::ProjectInfo,
                ResponseType::Complete,
            ),
        ]);
    }

    pub fn terminate(&mut self) {
        self.send_response(vec![Response::new(
            ResponseCode::SyntaxErrorInParametersOrArguments,
            ResponseMessage::Custom("Session terminated"),
            ResponseType::Complete,
        )]);
    }
}
