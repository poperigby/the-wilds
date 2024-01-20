use client_lib::ServerConnection;
use shared::Message;

fn main() -> std::io::Result<()> {
    let server = ServerConnection::connect("127.0.0.1:19773")?;

    let message = Message {
        message: "Hello, world!".to_string(),
        id: 0,
    };

    server.send_data(&message).unwrap();

    Ok(())
}
