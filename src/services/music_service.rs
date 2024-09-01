use rusqlite::{params, Result};
use crate::models::music_model::Music;

use super::init_db;


pub async fn get_song(id: u32) -> Result<Music>{
    let conn = init_db()?;
    let mut sql = conn.prepare("SELECT * FROM music WHERE id == ?")?;

    let mut song = sql.query_map([id], |row| {
        Ok(Music {
            id: row.get(0)?,
            song_path: row.get(1)?,
            title: row.get(2)?,
            artist: row.get(3)?,
            genre: row.get(4)?,
            duration: row.get(5)?,

        })
    })?;

    match song.next() {
        Some(song) => song,
        None => Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

pub async fn get_all_songs() -> Result<Vec<Music>>{
    let conn = init_db()?;
    let mut sql = conn.prepare("SELECT * FROM music")?;

    let songs = sql.query_map([], |row| {
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
        music.push(song?);
    }

    Ok(music)
}

pub async fn search_song(query: &str) -> Result<Vec<Music>>{
    let query = format!("%{}%", query);
    let conn = init_db()?;
    let mut sql = conn.prepare("SELECT * FROM music WHERE title LIKE ?")?;

    let songs = sql.query_map([query], |row| {
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
        music.push(song?);
    }

    Ok(music)
    
}

pub async fn add_song(music: Music) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "INSERT INTO music (song_path, title, artist, genre, duration) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            music.song_path,
            music.title,
            music.artist,
            music.genre,
            music.duration
        ],
    )?;

    Ok(())
}

pub async fn remove_all_songs() -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "DELETE FROM music",
        params![],
    )?;

    //reset primary key
    conn.execute(
        "DELETE FROM sqlite_sequence WHERE name='music'",
        params![],
    )?;

    Ok(())
}