use shared::Message;
use std::{
    io::BufReader,
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

    let message: Message = serde_json::from_reader(buf_reader).unwrap();

    dbg!(message);
}
