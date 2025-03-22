use crate::command::types::CommandType;
use crate::database::Database;
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
    pub fn from_env(env: &str) -> Self {
        dotenv::from_filename(env).ok();

        Self {
            host: dotenv::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not set"),
            port: dotenv::var("SERVER_PORT")
                .expect("SERVER_PORT not set")
                .parse()
                .unwrap(),
            debug: dotenv::var("DEBUG")
                .expect("DEBUG not set")
                .parse()
                .unwrap(),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        self.debug_log("Running database migrations".to_string());
        Database::run_migrations();

        if dotenv::var("SEED_DATABASE")
            .unwrap()
            .parse::<bool>()
            .unwrap()
        {
            self.debug_log("Seeding database".to_string());
            Database::seed();
        }

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

                    if !session.process(CommandType::from(&data)) {
                        break;
                    }
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
