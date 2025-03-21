mod command;
mod response;
mod server;
mod session;
mod database;
mod auth;

use server::Server;

#[tokio::main]
async fn main() {
    let server = Server::from_env(".env");
    server.run().await.unwrap();
}
