mod auth;
mod command;
mod connection;
mod constants;
mod database;
mod response;
mod server;
mod session;
mod filesystem;

use server::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::from_env(".env");
    server.run().await.unwrap();
}
