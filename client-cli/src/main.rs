use shared::Message;
use std::{io::prelude::*, net::TcpStream};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:19773");
}
