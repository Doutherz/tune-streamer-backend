use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Music {
    pub id: u32,
    pub title: Option<String>,
    pub song_path: String,
    pub artist: Option<String>,
    pub genre: Option<String>,
    pub duration: Option<u64>,
}