use crate::command::types::CommandType;
use crate::connection::{ControlFlowStatement, ExitMode};
use crate::constants::COMMAND_BUFFER_SIZE;
use crate::session::Session;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub struct ControlConnection {
    id: u16,
    address: String,
    socket: TcpStream,
    debug: bool,
    exit_mode: ExitMode,
}

impl ControlConnection {
    pub fn new(id: u16, address: String, socket: TcpStream, debug: bool) -> Self {
        Self {
            id,
            address,
            socket,
            debug,
            exit_mode: ExitMode::None,
        }
    }

    pub async fn handle(&mut self) {
        let mut buffer = [0; COMMAND_BUFFER_SIZE];

        let (mut reader, writer) = self.socket.split();

        let mut session = Session::new(writer);

        session.init();

        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => {
                    self.debug_log("Read 0 bytes from connection");
                    break;
                }
                Ok(bytes_read) => {
                    let data = String::from_utf8_lossy(&buffer[..bytes_read]);

                    let command_type = CommandType::from(data.as_ref());

                    if command_type.is_none() {
                        match self
                            .exit_mode
                            .get_control_flow_statement(&buffer[..bytes_read])
                        {
                            Some(ControlFlowStatement::Continue(exit_mode)) => {
                                if let Some(exit_mode) = exit_mode {
                                    self.exit_mode = exit_mode;
                                }
                                continue;
                            }
                            Some(ControlFlowStatement::Break) => break,
                            Some(ControlFlowStatement::TerminateAndBreak) => {
                                session.terminate();
                                break;
                            }
                            None => {}
                        }
                    }

                    if !session.process(command_type) {
                        break;
                    }
                }
                Err(e) => {
                    self.debug_log(format!("Error reading from connection: {}", e).as_str());
                    break;
                }
            }
        }

        self.debug_log("Connection closed");
    }

    fn debug_log(&self, message: &str) {
        if self.debug {
            println!(
                "Client id: {} address: {} -> {}",
                self.id, self.address, message
            );
        }
    }
}
