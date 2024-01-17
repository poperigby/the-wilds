use serde_json;
use shared::Message;
use std::{io::prelude::*, net::TcpStream};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:19773")?;

    let message = Message {
        message: "Hello, world!".to_string(),
        id: 0,
    };

    stream.write(&serde_json::to_vec(&message).unwrap());

    Ok(())
}
