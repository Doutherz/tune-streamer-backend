use tide::{Error, Request, Response, Result, StatusCode};

use crate::models::playlist_model::Playlist;
use crate::services::user_service::{self, get_session_user};
use crate::services::{self, playlist_service};

#[derive(serde::Deserialize)]
#[serde(default)]
struct PlaylistPerams {
    playlist_name: String,
    is_public: bool,
}

impl Default for PlaylistPerams {
    fn default() -> Self {
        Self {
            playlist_name: String::from("Untitled Playlist"),
            is_public: false,
        }
    }
}

pub async fn create_playlist(mut req: Request<()>) -> Result<Response> {
    let playlist: PlaylistPerams = req.body_json().await?;

    let session = match req.cookie("Session-Token") {
        Some(cookie) => cookie.value().to_string(),
        None => return Err(Error::from_str(StatusCode::Forbidden, "Have to be logged in to create playlist")),
    };

    let user = get_session_user(session.as_str()).await?;

    playlist_service::create_playlist(user, &playlist.playlist_name, playlist.is_public).await?;

    Ok(Response::builder(StatusCode::Created).build())
}

pub async fn get_playlist_music(req: Request<()>) -> Result<Response> {
    //check if playlist is public or if its users playlist
    let playlist: Playlist = get_playlist(req).await?.take_body().into_json().await?;
    let music = playlist_service::get_playlist_music(playlist).await?;

    let res = Response::builder(StatusCode::Ok)
        .body(tide::Body::from_json(&music)?)
        .build();

    Ok(res)
}

pub async fn add_music(mut req: Request<()>) -> Result<Response> {
    todo!()
}

pub async fn get_playlist(req: Request<()>) -> Result<Response> {
    let playlist_id: u32 = req.param("id")?.parse()?;

    let token = req.cookie("Session-Token");

    let playlist = playlist_service::get_playlist(playlist_id).await?;

    let res = Response::builder(StatusCode::Ok)
        .body(tide::Body::from_json(&playlist)?)
        .build();

    // if playlist is public return it, if not check if the current user is the creator of the playlist
    if playlist.is_public {
        return Ok(res);
    } else if  let Some(token) = token {
        let user = user_service::get_session_user(token.value()).await?;
        if user.id == playlist.id {
            return Ok(res);
        }
    } else {
        return Err(Error::from_str(StatusCode::Unauthorized, "Cannot access private playlist"));
    }


    Ok(Response::builder(StatusCode::Created).build())
}