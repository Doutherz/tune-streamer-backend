use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Music {
    pub id: u32,
    pub title: String,
    pub song_path: String,
    pub artist: String,
    pub genre: String,
    pub duration: i32,
}