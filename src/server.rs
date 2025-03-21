use crate::command::types::CommandType;
use crate::session::Session;
use std::error::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

const COMMAND_BUFFER_SIZE: usize = 1024;

pub struct Server {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

impl Server {
    pub fn new(host: String, port: u16, debug: bool) -> Self {
        Self { host, port, debug }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind((self.host.as_str(), self.port)).await?;
        let mut session_count = 0;

        self.debug_log(format!(
            "Server listening on {}",
            listener.local_addr().unwrap()
        ));

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            session_count += 1;

            self.debug_log(format!(
                "Accepted connection from {} with session id {}",
                socket.peer_addr().unwrap(),
                session_count
            ));

            tokio::spawn(async move {
                let mut buffer = [0; COMMAND_BUFFER_SIZE];

                let (mut reader, writer) = socket.into_split();

                let mut session = Session::new(session_count, writer);

                session.init();

                loop {
                    let bytes_read = reader.read(&mut buffer).await.unwrap();
                    let data = String::from_utf8_lossy(&buffer[..bytes_read]);

                    session.process(CommandType::from(&data))
                }
            });
        }
    }

    pub fn debug_log(&self, message: String) {
        if self.debug {
            println!("{}", message);
        }
    }
}
