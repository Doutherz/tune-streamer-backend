use tide::Server;

use crate::handlers::user_handler;

pub fn user_routes(app: &mut Server<()>) {

    app.at("/").post(user_handler::add_user);

    app.at("/delete").post(|_| async {
        //check authenticated
        //remove user
        Ok("created user")
    });

    app.at("/login").post(user_handler::login);

    app.at("/logout").post(|_| async {
        //user logout
        //check is authenticated
        Ok("created user")
    });
}