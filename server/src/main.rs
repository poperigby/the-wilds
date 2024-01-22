use server::Server;

fn main() {
    let server = Server::bind("127.0.0.1:19773");
    server.listen();
}
