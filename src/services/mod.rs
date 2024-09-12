use rusqlite::{Connection, Result};

use crate::DATABASE_URL;

//interact with database
pub mod music_service;
pub mod user_service;
pub mod playlist_service;

pub fn init_db() -> Result<Connection> {


    let conn = Connection::open(*DATABASE_URL)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS music (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        song_path TEXT NOT NULL,
        title TEXT,
        artist TEXT,
        genre TEXT,
        duration INTEGER
        )",
        [],
    )?;

    conn.execute("CREATE TABLE IF NOT EXISTS playlists (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        user_id INTEGER NOT NULL,
        is_public INTEGER NOT NULL DEFAULT 0
        )", []
    )?;

    conn.execute("CREATE TABLE IF NOT EXISTS playlists_songs (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        playlist_id INTEGER NOT NULL,
        song_id INTEGER NOT NULL
        )", []
    )?;

    conn.execute("CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        username TEXT NOT NULL,
        password TEXT NOT NULL,
        salt TEXT NOT NULL,
        session_token TEXT
        )", []
    )?;

    

    Ok(conn)
}