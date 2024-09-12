use std::env;
use tide::{log, Error, StatusCode};
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
        Err(_) => "0.0.0.0",
    }
});

pub static PORT: Lazy<&'static str> = Lazy::new(|| {
    match env::var("PORT") {
        Ok(value) => Box::leak(value.into_boxed_str()),
        Err(_) => "8080",
    }
});



#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::new();

    routes::routes(&mut app);

    utils::logging::handle_logging(&mut app);

    load_music().await.unwrap_or_else(|err| log::error!("{}",err));

    println!("Running on: {}:{}", *IP, *PORT);
    app.listen(format!("{}:{}", *IP,  *PORT)).await?;
    

    Ok(())
}
