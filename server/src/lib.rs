pub mod character_sheet;

use shared::Message;
use std::{
    io::BufReader,
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

    /// Begin listening for incoming connections.
    pub fn listen(&self) -> Result<(), std::io::Error> {
        for stream in self.listener.incoming() {
            self.handle_client(stream?);
        }

        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        // TODO: Send back an error message through the stream, if we can't
        // deserialize the Message, instead of crashing the server :|
        let message: serde_json::Result<Message> = serde_json::from_reader(buf_reader);

        match message {
            Ok(m) => {
                match m {
                    Message::Get(get_message) => dbg!(get_message),
                };
            }
            Err(e) => {
                dbg!(e);
            }
        };
    }
}
