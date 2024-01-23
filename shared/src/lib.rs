pub mod character_sheet;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct Message {
    pub message: String,
    pub id: usize,
}
