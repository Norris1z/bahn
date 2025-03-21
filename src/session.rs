use crate::command::Command;
use crate::command::types::CommandType;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
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

    pub fn process(&mut self, command_type: Option<CommandType>) -> bool {
        self.send_response(Command::new(command_type, &self.user).handle())
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
}
