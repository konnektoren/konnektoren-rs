use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Map {
    pub background: String,
    pub width: u32,
    pub height: u32,
}
