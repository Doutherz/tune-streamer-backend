use tide::{Error, Request, Response, Result, StatusCode};
use async_std::{fs::File, io::ReadExt};
use crate::services::music_service;

#[derive(serde::Deserialize)]
#[serde(default)]
struct SongPerams {
    id: String,
}

impl Default for SongPerams {
    fn default() -> Self {
        Self {
            id: "".to_string(),
        }
    }
}

pub async fn play_song(req: Request<()>) -> Result<Response> {
    let song_perams: SongPerams = req.query()?;
    let song = music_service::get_song(song_perams.id).await;
    let song = match song {
        Ok(song) => {song},
        Err(e) => {
            return Err(Error::from_str(StatusCode::NotFound, e));
        }
    };
    let song = File::open(format!("./Tune-Streamer_music/{}", song.song_path)).await;

    match song {
        Ok(mut file) => {
            let mut contents = vec![];
            if let Err(e) = file.read_to_end(&mut contents).await {
                return Err(Error::from_str(StatusCode::NotFound, format!("Failed to read file - {}", e)));
            }
            Ok(Response::builder(StatusCode::Ok)
                .header("Content-Type","audio/mpeg")
                .body(contents)
                .build())
        }
        Err(e) => {
            Err(Error::from_str(StatusCode::NotFound, format!("Song not found - {}", e)))
        }
    }
}