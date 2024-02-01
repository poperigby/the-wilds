use async_std::main;
use human_panic::setup_panic;
use server::Server;

#[main]
async fn main() {
    setup_panic!();

    let server = Server::bind("127.0.0.1:19773").unwrap();
    server.listen().await.unwrap();
}
