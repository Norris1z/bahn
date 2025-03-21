use crate::command::Command;
use crate::command::types::CommandType;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseType};
use crate::user::User;
use std::cell::RefCell;
use tokio::net::tcp::OwnedWriteHalf;

pub struct Session {
    #[allow(dead_code)]
    id: u16,
    user: RefCell<User>,
    socket_writer: OwnedWriteHalf,
}

impl Session {
    pub fn new(id: u16, socket_writer: OwnedWriteHalf) -> Self {
        Self {
            id,
            user: RefCell::new(User::new()),
            socket_writer,
        }
    }

    pub fn process(&mut self, command_type: Option<CommandType>) {
        let command = Command::new(command_type, &self.user);
        self.send_response(command.handle());
    }

    fn send_response(&mut self, response: Response) -> &mut Self {
        self.socket_writer
            .try_write(response.to_string().as_bytes())
            .unwrap();

        self
    }

    pub fn init(&mut self) {
        self.send_response(Response::new(
            ResponseCode::Success,
            ResponseMessage::Greeting,
            ResponseType::Partial,
        ))
        .send_response(Response::new(
            ResponseCode::Success,
            ResponseMessage::ProjectInfo,
            ResponseType::Complete,
        ));
    }
}
