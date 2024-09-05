

use rusqlite::{params, Result};
use crate::models::{music_model::Music, playlist_model::Playlist, user_model::User};

use super::init_db;

pub async fn create_playlist(user: User, playlist_name: &str, is_public: bool) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "INSERT INTO playlists (name, user_id, is_public) VALUES (?1, ?2, ?3)",
        params![
            playlist_name,
            user.id,
            is_public,
        ],
    )?;

    Ok(())
}

pub async fn get_playlist(playlist_id: u32) -> Result<Playlist> {
    let conn = init_db()?;
    let mut sql = conn.prepare("SELECT * FROM playlists WHERE id = ?")?;

    let mut playlist = sql.query_map([playlist_id], |row| {
        Ok(Playlist {
            id: row.get(0)?,
            name: row.get(1)?,
            user_id: row.get(2)?,
            is_public: row.get(3)?,
        })
    })?;

    match playlist.next() {
        Some(playlist) => playlist,
        None => Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

pub async fn get_playlist_music(playlist: Playlist) -> Result<Vec<Music>> {
    let conn = init_db()?;
    let mut sql = conn.prepare("SELECT music.id, song_path, title, artist, genre, duration FROM playlists_songs INNER JOIN music ON playlists_songs.song_id = music.id WHERE playlist_id = ?")?;

    let songs = sql.query_map([playlist.id], |row| {
        Ok(Music {
            id: row.get(0)?,
            song_path: row.get(1)?,
            title: row.get(2)?,
            artist: row.get(3)?,
            genre: row.get(4)?,
            duration: row.get(5)?,

        })
    })?;

    let mut music = Vec::new();

    for song in songs {
        let mut song = song?;
        song.song_path = format!("{}", song.id);
        music.push(song);
    }

    Ok(music)
}

pub async fn search_playlist(query: &str, user: User) -> Result<Vec<Playlist>> {
    let query = format!("%{}%", query);
    let conn = init_db()?;
    let mut sql = conn.prepare("SELECT * FROM playlists WHERE name LIKE ?1 AND is_public = true OR user_id = ?2")?;

    let playlists = sql.query_map([query, user.id.to_string()], |row| {
        Ok(Playlist {
            id: row.get(0)?,
            name: row.get(1)?,
            user_id: row.get(2)?,
            is_public: row.get(3)?,
        })
    })?;

    let mut check_playlists = Vec::new();
    for playlist in playlists {
        check_playlists.push(playlist?);
    }

    Ok(check_playlists)
}

pub async fn get_user_playlists(user: User) -> Result<Vec<Playlist>> {
    let conn = init_db()?;
    let mut sql = conn.prepare("SELECT * FROM playlists WHERE user_id = ?")?;

    let playlists = sql.query_map([user.id], |row| {
        Ok(Playlist {
            id: row.get(0)?,
            name: row.get(1)?,
            user_id: row.get(2)?,
            is_public: row.get(3)?,
        })
    })?;

    let mut check_playlists = Vec::new();
    for playlist in playlists {
        check_playlists.push(playlist?);
    }

    Ok(check_playlists)
}

pub async fn add_song(song: Music, playlist: Playlist) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "INSERT INTO playlists_songs (playlist_id, song_id) VALUES (?1, ?2)",
        params![
            playlist.id,
            song.id,
        ],
    )?;

    Ok(())
}

pub async fn update_playlist(playlist: Playlist) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "UPDATE playlists SET name = ?2, is_public = ?3 WHERE id = ?1",
        params![
            playlist.id,
            playlist.name,
            playlist.is_public,
        ],
    )?;
    Ok(())
}