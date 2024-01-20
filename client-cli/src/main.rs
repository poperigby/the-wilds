use serde::{de::DeserializeOwned, Serialize};
use shared::Message;
use std::{
    io::prelude::*,
    net::{TcpStream, ToSocketAddrs},
};

fn main() -> std::io::Result<()> {
    let connection = Connection::new("127.0.0.1:19773")?;

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

/// A TCP connection to a The Wilds server
impl Connection {
    /// Create a new connection to a server at the given address
    fn new<A: ToSocketAddrs>(address: A) -> std::io::Result<Self> {
        Ok(Connection {
            stream: TcpStream::connect(address)?,
        })
    }

    /// Send a serializeable struct over the connection
    fn send_data<D: DeserializeOwned + Serialize>(mut self, data: &D) {
        self.stream.write(&serde_json::to_vec(&data).unwrap());
    }
}
