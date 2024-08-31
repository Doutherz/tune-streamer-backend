use rusqlite::{Connection, Result};
use crate::models::music_model::Music;
fn init_db() -> Result<Connection> {
    let conn = Connection::open("./db/database.db")?;
    Ok(conn)
}

pub async fn get_song(id: String) -> Result<Music>{
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