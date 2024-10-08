use tide::Server;
mod user_route;
mod song_route;
mod playlist_route;
//for routes
pub fn routes(app: &mut Server<()>) {
    app.at("/api").nest({
        let mut api = tide::new();

        api.at("/").get(|_| async {
            Ok("Version 0.1")
        });

        api.at("/users").nest({
            let mut user_route = tide::new();

            user_route::user_routes(&mut user_route);

            user_route
        });

        api.at("/music").nest({
            let mut song_route = tide::new();

            song_route::song_routes(&mut song_route);

            song_route
        });

        api.at("/playlists").nest({
            let mut playlist_route = tide::new();

            playlist_route::playlist_routes(&mut playlist_route);

            playlist_route
        });

        api
    });
}