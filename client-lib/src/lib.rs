use serde::{de::DeserializeOwned, Serialize};
use std::{
    io::prelude::*,
    net::{TcpStream, ToSocketAddrs},
};

pub struct Connection {
    stream: TcpStream,
}

/// A TCP connection to a The Wilds server
impl Connection {
    /// Create a new connection to a server at the given address
    pub fn new<A: ToSocketAddrs>(address: A) -> std::io::Result<Self> {
        Ok(Connection {
            stream: TcpStream::connect(address)?,
        })
    }

    /// Send a serializeable struct over the connection
    pub fn send_data<D: DeserializeOwned + Serialize>(mut self, data: &D) {
        let json_data = &serde_json::to_vec(&data).unwrap();
        self.stream.write(json_data);
    }
}
