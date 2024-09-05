use tide::{http::Url, Error, Request, Response, Result, StatusCode};
use async_std::{fs::File, io::ReadExt};
use crate::{models::music_model::Music, services::music_service};


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

//get song should not have local path to the mp3 file but instead have the path to the url song
pub async fn get_song(req: Request<()>) -> Result<Response> {
    let song_id: &str = req.param("id")?;
    let song = music_service::get_song(song_id.parse()?).await;
    let song = match song {
        Ok(song) => {song},
        Err(e) => {
            return Err(Error::from_str(StatusCode::NotFound, e));
        }
    };

    let mut song = vec![song];
    change_filepath_to_urlpath(&mut song, req.url());
    let song = &song[0];


    Ok(Response::builder(StatusCode::Ok)
        .body(tide::Body::from_json(&song)?)
        .build()
    )
}

pub async fn search_song(req: Request<()>) -> Result<Response> {
    let song_perams:QueryPerams = req.query()?;
    let songs = music_service::search_song(song_perams.q.as_str()).await;
    let mut songs = match songs {
        Ok(song) => {song},
        Err(e) => {
            return Err(Error::from_str(StatusCode::NotFound, e));
        }
    };

    change_filepath_to_urlpath(&mut songs, req.url());

    Ok(Response::builder(StatusCode::Ok)
        .body(tide::Body::from_json(&songs)?)
        .build()
    )
}

pub async fn all_songs(req: Request<()>) -> Result<Response> {
    let songs = music_service::get_all_songs().await;
    let mut songs = match songs {
        Ok(song) => {song},
        Err(e) => {
            return Err(Error::from_str(StatusCode::NotFound, e));
        }
    };

    change_filepath_to_urlpath(&mut songs, req.url());

    Ok(Response::builder(StatusCode::Ok)
        .body(tide::Body::from_json(&songs)?)
        .build()
    )
}

pub fn change_filepath_to_urlpath (music: &mut Vec<Music>, url: &Url){
    let host = if let Some(port) = url.port()  {
        format!("{}://{}:{}", url.scheme(), url.host_str().unwrap_or(""), port)
    } else {
        format!("{}://{}", url.scheme(), url.host_str().unwrap_or(""))
    };

    let path = "/api/music/play".to_string();

    for song in music.iter_mut() {
        song.song_path = format!("{}{}/{}", host, path, song.id);
    }
}