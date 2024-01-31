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
    pub fn bind<A: ToSocketAddrs>(address: A) -> Self {
        Server {
            listener: TcpListener::bind(address).unwrap(),
        }
    }

    /// Begin listening for incoming connections.
    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            self.handle_client(stream.unwrap());
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let message: Message = serde_json::from_reader(buf_reader).unwrap();

        match message {
            Message::Get(get_message) => dbg!(get_message),
        };
    }
}
