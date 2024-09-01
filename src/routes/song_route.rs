use tide::Server;
use crate::handlers::music_handler;

pub fn song_routes(app: &mut Server<()>) {
    app.at("/").get(music_handler::all_songs);
    app.at("/play").get(music_handler::play_song);
    app.at("/get").get(music_handler::get_song);
    app.at("/search").get(music_handler::search_song);
}