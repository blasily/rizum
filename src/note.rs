use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub tick: u32,
    pub lane: u8,
}
