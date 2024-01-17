use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:19773");
}
