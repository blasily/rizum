use crate::Note;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Chart {
    pub version: u32,
    pub notes: Vec<Note>,
}
