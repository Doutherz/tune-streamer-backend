use std::env;
use tide::http::headers::HeaderValue;
use tide::log;
use tide::security::{CorsMiddleware, Origin};
use utils::load_music::load_music;
use once_cell::sync::Lazy;

mod routes;
mod handlers;
mod services;
mod models;
mod utils;
mod middleware;

pub static DATABASE_URL: Lazy<&'static str> = Lazy::new(|| {
    match env::var("DATABASE_URL") {
        Ok(value) => Box::leak(value.into_boxed_str()),
        Err(_) => "./db/database.db",
    }
});

pub static MUSICDATA_URL: Lazy<&'static str> = Lazy::new(|| {
    match env::var("MUSICDATA_URL") {
        Ok(value) => Box::leak(value.into_boxed_str()),
        Err(_) => "./Tune-Streamer_music",
    }
});

pub static IP: Lazy<&'static str> = Lazy::new(|| {
    match env::var("IP") {
        Ok(value) => Box::leak(value.into_boxed_str()),
        Err(_) => "localhost",
    }
});

pub static PORT: Lazy<&'static str> = Lazy::new(|| {
    match env::var("PORT") {
        Ok(value) => Box::leak(value.into_boxed_str()),
        Err(_) => "8080",
    }
});

pub static CORS: Lazy<&'static str> = Lazy::new(|| {
    match env::var("CORS") {
        Ok(value) => Box::leak(value.into_boxed_str()),
        Err(_) => "http://localhost:5173",
    }
});



#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::new();

    routes::routes(&mut app);

    let cors = CorsMiddleware::new()
        .allow_origin(Origin::from(*CORS))
        .allow_methods("GET, POST, DELETE".parse::<HeaderValue>()?)
        .allow_credentials(true);

    app.with(cors);

    load_music().await.unwrap_or_else(|err| log::error!("{}",err));

    println!("Running on: {}:{}", "0.0.0.0", *PORT);
    app.listen(format!("{}:{}", "0.0.0.0",  *PORT)).await?;
    

    Ok(())
}
