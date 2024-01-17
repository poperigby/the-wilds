use shared::Message;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() -> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:19773")?;

    for stream in listener.incoming() {
        dbg!(stream?);
    }

    Ok(())
}
