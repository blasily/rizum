use crate::{Metadata, Note, Timing};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Chart {
    pub version: u32,
    pub metadata: Metadata,
    pub timing: Timing,
    pub notes: Vec<Note>,
}
