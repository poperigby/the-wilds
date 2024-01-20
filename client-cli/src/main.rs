use client_lib::Connection;
use shared::Message;

fn main() -> std::io::Result<()> {
    let connection = Connection::new("127.0.0.1:19773")?;

    let message = Message {
        message: "Hello, world!".to_string(),
        id: 0,
    };

    connection.send_data(&message);

    Ok(())
}
