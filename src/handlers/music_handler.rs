use tide::{Error, Request, Response, Result, StatusCode};
use async_std::{fs::File, io::ReadExt};
use crate::services::music_service;


#[derive(serde::Deserialize)]
struct QueryPerams {
    q: String,
}



pub async fn play_song(req: Request<()>) -> Result<Response> {
    let song_id: &str = req.param("id")?;
    let song = music_service::get_song(song_id.parse()?).await;
    let song = match song {
        Ok(song) => {song},
        Err(e) => {
            return Err(Error::from_str(StatusCode::NotFound, e));
        }
    };
    let song = File::open(format!("{}", song.song_path)).await;

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

pub async fn get_song(req: Request<()>) -> Result<Response> {
    let song_id: &str = req.param("id")?;
    let song = music_service::get_song(song_id.parse()?).await;
    let song = match song {
        Ok(song) => {song},
        Err(e) => {
            return Err(Error::from_str(StatusCode::NotFound, e));
        }
    };

    Ok(Response::builder(StatusCode::Ok)
        .body(tide::Body::from_json(&song)?)
        .build()
    )
}

pub async fn search_song(req: Request<()>) -> Result<Response> {
    let song_perams:QueryPerams = req.query()?;
    let songs = music_service::search_song(song_perams.q.as_str()).await;
    let songs = match songs {
        Ok(song) => {song},
        Err(e) => {
            return Err(Error::from_str(StatusCode::NotFound, e));
        }
    };

    Ok(Response::builder(StatusCode::Ok)
        .body(tide::Body::from_json(&songs)?)
        .build()
    )
}

pub async fn all_songs(_req: Request<()>) -> Result<Response> {
    let songs = music_service::get_all_songs().await;
    let songs = match songs {
        Ok(song) => {song},
        Err(e) => {
            return Err(Error::from_str(StatusCode::NotFound, e));
        }
    };

    Ok(Response::builder(StatusCode::Ok)
        .body(tide::Body::from_json(&songs)?)
        .build()
    )
}