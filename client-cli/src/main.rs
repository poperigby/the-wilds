use serde::{de::DeserializeOwned, Serialize};
use shared::Message;
use std::{
    io::prelude::*,
    net::{TcpStream, ToSocketAddrs},
};

fn main() -> std::io::Result<()> {
    let connection = Connection::build("127.0.0.1:17732")?;

    let message = Message {
        message: "Hello, world!".to_string(),
        id: 0,
    };

    connection.send_data(&message);

    Ok(())
}

struct Connection {
    stream: TcpStream,
}

impl Connection {
    fn build<A: ToSocketAddrs>(address: A) -> std::io::Result<Self> {
        Ok(Connection {
            stream: TcpStream::connect(address)?,
        })
    }

    fn send_data<T: DeserializeOwned + Serialize>(mut self, data: &T) {
        self.stream.write(&serde_json::to_vec(&data).unwrap());
    }
}
