use tide::Server;

pub fn user_routes(app: &mut Server<()>) {
    app.at("/").get(|_| async {
        Ok("Users")
    });
}