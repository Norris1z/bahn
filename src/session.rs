use crate::command::Command;
use crate::command::types::CommandType;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
use std::cell::RefCell;
use tokio::net::tcp::WriteHalf;
use user::User;

pub mod user;

pub struct Session<'client_connection> {
    user: RefCell<User>,
    socket_writer: WriteHalf<'client_connection>,
}

impl<'client_connection> Session<'client_connection> {
    pub fn new(socket_writer: WriteHalf<'client_connection>) -> Self {
        Self {
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

    pub fn terminate(&mut self) {
        self.send_response(vec![Response::new(
            ResponseCode::SyntaxErrorInParametersOrArguments,
            ResponseMessage::Custom("Session terminated"),
            ResponseType::Complete,
        )]);
    }
}
