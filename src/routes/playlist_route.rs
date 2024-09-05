use tide::Server;
use crate::{handlers::playlist_handler, middleware::authentication};

pub fn playlist_routes(app: &mut Server<()>) {
    app.at("/").with(authentication::is_auth).post(playlist_handler::create_playlist);
    app.at("/:id").get(playlist_handler::get_playlist);
    app.at("/:id/music").get(playlist_handler::get_playlist_music);
    app.at("/:id/:music_id").with(authentication::is_auth).post(playlist_handler::add_music);
}