use client_lib::ServerConnection;
use shared::{GetMessage, Message};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn send_message() {
    let server = ServerConnection::connect("127.0.0.1:19773").unwrap_throw();

    let message = Message::Get(GetMessage {
        message: "Hello, world!".to_string(),
        id: 0,
    });

    server.send(&message).unwrap_throw();
}
