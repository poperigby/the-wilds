use shared::Message;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:19773")?;

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    dbg!(buf_reader);
}
