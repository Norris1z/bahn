mod command;
mod response;
mod server;
mod session;
mod user;

use server::Server;

#[tokio::main]
async fn main() {
    let server = Server::from_env(".env");
    server.run().await.unwrap();
}
