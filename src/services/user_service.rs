use super::init_db;
use rusqlite::{params, Result};
use pwhash::bcrypt;
use rand::prelude::*;

use crate::models::user_model::{NewUser, User};

pub async fn username_taken(username: &str) -> Result<bool> {
    let conn = init_db()?;
    let mut sql = conn.prepare("SELECT * FROM users WHERE username = ?")?;

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

pub async fn add_user(user: NewUser) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "INSERT INTO users (username, password, salt) VALUES (?1, ?2, ?3)",
        params![
            user.username,
            user.password,
            user.salt,
        ],
    )?;

    Ok(())
}

pub fn get_hash(password: &str, salt: &str) -> pwhash::Result<String>{
    Ok(bcrypt::hash(format!("{}{}", password, salt))?)
}

pub fn gen_salt(len: usize) -> String {
    let mut salt = vec![0u8; len];
    let mut rng = rand::thread_rng();
    rng.fill(&mut salt[..]);
    salt.iter().map(|byte| format!("{:02x}", byte)).collect()
}

//true is password correct
//false is password not correct
pub async fn authenticate(username: &str, password: &str) -> Result<bool> {
    let conn = init_db()?;
    let mut sql = conn.prepare("SELECT * FROM users WHERE username = ?")?;

    let mut user = sql.query_map([username], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            salt: row.get(3)?,
            session_token: row.get(4)?,
        })
    })?;

    let user = match user.next() {
        Some(user) => Ok(user?),
        None => Err(rusqlite::Error::QueryReturnedNoRows),
    }?;

    Ok(bcrypt::verify(format!("{}{}", password, user.salt), &user.password))
} 

pub async fn gen_token(username: &str) -> Result<String> {
    let conn = init_db()?;
    let token = gen_salt(20);
    conn.execute(
        "UPDATE users SET session_token = ?1 WHERE username = ?2",
        params![token, username],
    )?;

    Ok(token)
}

pub async fn remove_token(token: &str) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "UPDATE users SET session_token = NULL WHERE session_token = ?1",
        params![token],
    )?;

    Ok(())
}


pub async fn get_session_user(token: &str) -> Result<User>{
    let conn = init_db()?;
    let mut sql = conn.prepare("SELECT * FROM users WHERE session_token = ?")?;

    let mut user = sql.query_map([token], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            salt: row.get(3)?,
            session_token: row.get(4)?,
        })
    })?;

    let user = match user.next() {
        Some(user) => Ok(user?),
        None => Err(rusqlite::Error::QueryReturnedNoRows),
    }?;

    Ok(user)
}