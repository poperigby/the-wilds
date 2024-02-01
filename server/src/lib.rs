pub mod character_sheet;

use shared::{ErrorMessage, Message};
use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn bind<A: ToSocketAddrs>(address: A) -> std::io::Result<Self> {
        Ok(Server {
            listener: TcpListener::bind(address)?,
        })
    }

    /// Listen for incoming TCP connections.
    pub async fn listen(&self) -> Result<(), std::io::Error> {
        for stream in self.listener.incoming() {
            self.handle_client(stream?);
        }

        Ok(())
    }

    // TODO: Handle this in ClientConnection
    fn handle_client(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let message: serde_json::Result<Message> = serde_json::from_reader(buf_reader);

        match message {
            Ok(m) => {
                match m {
                    Message::Get(get_message) => {
                        dbg!(get_message);
                    }
                    Message::Error(error_message) => {
                        dbg!(error_message);
                    }
                };
            }
            Err(e) => {
                // Send back an error message through the stream, if we can't
                // deserialize the Message.
                let error_message = Message::Error(ErrorMessage {
                    message: e.to_string(),
                });
                let json_data = &serde_json::to_vec(&error_message).unwrap();
                stream.write_all(json_data).unwrap();

                // TODO: Tell which client the error comes from
                log::error!("Error while deserializing message: {e}");
            }
        };
    }
}

struct ClientConnection {
    stream: TcpStream,
}

impl ClientConnection {
    // fn send()
}

impl From<TcpStream> for ClientConnection {
    fn from(stream: TcpStream) -> Self {
        Self { stream }
    }
}
