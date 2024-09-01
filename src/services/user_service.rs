use super::init_db;
use rusqlite::Result;

use crate::models::user_model::User;

fn username_taken(username: &str) -> Result<bool>{
    let conn = init_db()?;
    let mut sql = conn.prepare("SELECT * FROM user WHERE id == ?")?;

    let mut user = sql.query_map([username], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            salt: row.get(3)?,
            session_token: row.get(4)?,
        })
    })?;

    match user.next() {
        Some(_user) => Ok(true),
        None => Ok(false),
    }
}