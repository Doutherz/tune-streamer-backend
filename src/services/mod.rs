use rusqlite::{Connection, Result};

//interact with database
pub mod music_service;
pub mod user_service;

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("./db/database.db")?;
    Ok(conn)
}