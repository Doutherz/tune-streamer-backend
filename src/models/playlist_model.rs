use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub id: u32,
    pub name: String,
    pub user_id: u32,
    pub is_public: bool,
}