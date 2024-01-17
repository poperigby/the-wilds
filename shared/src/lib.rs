use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub struct Message {
    pub message: String,
    pub id: usize,
}
