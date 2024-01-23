use serde::{Deserialize, Serialize};

/// A message that can be sent between the client and server.
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum Message {
    /// Request some state from the server.
    Get(GetMessage),
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct GetMessage {
    pub message: String,
    pub id: usize,
}
