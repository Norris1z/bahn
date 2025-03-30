use crate::command::Command;
use crate::command::context::CommandContext;
use crate::command::types::CommandType;
use crate::connection::CommunicationChannel;
use crate::connection::DataTransferStatus;
use crate::response::codes::ResponseCode;
use crate::response::messages::ResponseMessage;
use crate::response::{Response, ResponseCollection, ResponseType};
use std::cell::{Cell, RefCell};
use tokio::net::tcp::OwnedWriteHalf;
use user::User;

pub mod user;

pub struct Session {
    user: RefCell<User>,
    socket_writer: OwnedWriteHalf,
    data_connection_created: Cell<bool>,
    communication_channel: RefCell<CommunicationChannel<ResponseCollection, DataTransferStatus>>,
}

impl Session {
    pub fn new(socket_writer: OwnedWriteHalf) -> Self {
        Self {
            user: RefCell::new(User::new()),
            socket_writer,
            data_connection_created: Cell::new(false),
            communication_channel: RefCell::new(CommunicationChannel::new(None, None)),
        }
    }

    pub fn process(&mut self, command_type: Option<CommandType>) -> bool {
        let is_data_command = if command_type.is_some() {
            command_type
                .as_ref()
                .unwrap()
                .should_send_via_data_connection()
        } else {
            false
        };

        let command = Command::new(command_type);
        let context = CommandContext::new(
            &self.user,
            &self.data_connection_created,
            &self.communication_channel,
        );

        let response = command.handle(context);

        if is_data_command {
            self.send_data_response(response)
        } else {
            self.send_response(response)
        }
    }

    fn send_response(&mut self, responses: ResponseCollection) -> bool {
        responses.iter().for_each(|response| {
            println!("Sending response: {:?}", response.to_string());
            self.socket_writer
                .try_write(response.to_string().as_bytes())
                .unwrap();
        });

        let last_response = responses.last().unwrap();

        !last_response.is_terminate()
    }

    fn send_data_response(&mut self, responses: ResponseCollection) -> bool {
        if !self.data_connection_created.get() {
            return self.send_response(vec![Response::new(
                ResponseCode::CantOpenDataConnection,
                ResponseMessage::Custom("Use PORT or PASV first"),
                ResponseType::Complete,
            )]);
        }

        self.send_response(vec![Response::new(
            ResponseCode::StartingDataTransfer,
            ResponseMessage::Custom("Starting data transfer"),
            ResponseType::Complete,
        )]);

        let failed_response = vec![Response::new(
            ResponseCode::ConnectionClosedTransferAborted,
            ResponseMessage::Custom("Transfer aborted and connection closed"),
            ResponseType::Complete,
        )];

        if self
            .communication_channel
            .borrow()
            .sender
            .as_ref()
            .unwrap()
            .send(responses)
            .is_err()
        {
            return self.send_response(failed_response);
        }

        let status = self
            .communication_channel
            .borrow()
            .receiver
            .as_ref()
            .unwrap()
            .recv();

        if status.is_err() || status.unwrap() == DataTransferStatus::Failed {
            return self.send_response(failed_response);
        }

        self.cleanup_data_connection();

        self.send_response(vec![Response::new(
            ResponseCode::ClosingDataConnection,
            ResponseMessage::Custom("Operation successful"),
            ResponseType::Complete,
        )])
    }

    fn cleanup_data_connection(&mut self) {
        self.data_connection_created.replace(false);
        let mut channel = self.communication_channel.borrow_mut();
        *channel = CommunicationChannel::new(None, None);
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
