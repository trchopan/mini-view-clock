use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteHeader {
    pub emoji: Option<char>,
    pub content: String,
}
