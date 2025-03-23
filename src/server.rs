use crate::connection::ControlConnection;
use crate::database::Database;
use std::error::Error;
use tokio::net::TcpListener;

pub struct Server {
    pub host: String,
    pub port: u16,
    pub debug: bool,
    connection_count: u16,
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
            connection_count: 0,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
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

        self.debug_log(format!(
            "Server listening on {}",
            listener.local_addr().unwrap()
        ));

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            self.connection_count += 1;
            let address = socket.peer_addr().unwrap();

            self.debug_log(format!(
                "Accepted connection from {} with session id {}",
                address, self.connection_count
            ));

            let mut control_connection = ControlConnection::new(
                self.connection_count,
                address.to_string(),
                socket,
                self.debug,
            );

            tokio::spawn(async move {
                control_connection.handle().await;
            });

            self.debug_log(format!(
                "Connection with session id {} is ready to handle commands",
                self.connection_count
            ));
        }
    }

    fn debug_log(&self, message: String) {
        if self.debug {
            println!("{}", message);
        }
    }
}
