use shared::Message;
use std::{io::prelude::*, net::ToSocketAddrs};
use thiserror::Error;

/// A TCP connection to a game server
pub struct ServerConnection {
    socket: Stream,
}

enum Stream {
    Native(std::net::TcpStream),
    Web(websocket::stream::sync::TcpStream),
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
        if cfg!(feature = "web") {
            Ok(ServerConnection {
                socket: Stream::Web(websocket::stream::sync::TcpStream::connect(address)?),
            })
        } else {
            Ok(ServerConnection {
                socket: Stream::Native(std::net::TcpStream::connect(address)?),
            })
        }
    }

    /// Send a Message over the connection
    pub fn send(&mut self, message: &Message) -> Result<(), SendDataError> {
        let json_data = &serde_json::to_vec(&message)?;

        match &mut self.socket {
            Stream::Native(stream) => stream.write_all(json_data)?,
            Stream::Web(stream) => stream.write_all(json_data)?,
        }

        Ok(())
    }
}
