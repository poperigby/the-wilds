use serde::{de::DeserializeOwned, Serialize};
use std::{
    io::prelude::*,
    net::{TcpStream, ToSocketAddrs},
};
use thiserror::Error;

/// A TCP connection to a game server
pub struct ServerConnection {
    stream: TcpStream,
}

#[derive(Error, Debug)]
pub enum SendDataError {
    #[error("Failed to serialize data")]
    SerializationError(#[from] serde_json::Error),
    #[error("Failed to write to TCP stream")]
    StreamWriteError(#[from] std::io::Error),
}

impl ServerConnection {
    /// Attempt to establish a connection to a game server at the give `address`.
    ///
    /// # Examples
    ///
    /// ```
    /// use client_lib::ServerConnection;
    ///
    /// let server = ServerConnection::connect("127.0.0.1:19773");
    /// ```
    pub fn connect<A: ToSocketAddrs>(address: A) -> std::io::Result<Self> {
        Ok(ServerConnection {
            stream: TcpStream::connect(address)?,
        })
    }

    /// Send a serializeable struct over the connection
    pub fn send_data<D: DeserializeOwned + Serialize>(
        mut self,
        data: &D,
    ) -> Result<(), SendDataError> {
        let json_data = &serde_json::to_vec(&data)?;
        self.stream.write_all(json_data)?;

        Ok(())
    }
}
