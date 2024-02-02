use client_lib::ServerConnection;
use shared::{GetMessage, Message};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn send_message() -> Result<(), JsError> {
    let server = ServerConnection::connect("0.0.0.0:19773");

    match server {
        Ok(mut s) => {
            let message = Message::Get(GetMessage {
                message: "Hello, world!".to_string(),
                id: 0,
            });

            s.send(&message).expect_throw("Unable to send message");

            Ok(())
        }
        Err(e) => Err(JsError::new(&e.to_string())),
    }
}
