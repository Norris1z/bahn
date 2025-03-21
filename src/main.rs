mod command;
mod response;
mod server;
mod session;
mod user;

use server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new(String::from("127.0.0.1"), 9099, true);
    server.run().await.unwrap();
}
