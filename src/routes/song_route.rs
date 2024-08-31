use tide::Server;
use crate::handlers::music_handler;

pub fn song_routes(app: &mut Server<()>) {
    app.at("/play").get(music_handler::play_song);
}