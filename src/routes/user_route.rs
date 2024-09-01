use tide::Server;

pub fn user_routes(app: &mut Server<()>) {
    app.at("/").post(|_| async {
        //create user
        Ok("created user")
    });

    app.at("/delete").post(|_| async {
        //check authenticated
        //remove user
        Ok("created user")
    });

    app.at("/login").post(|_| async {
        //user login
        Ok("created user")
    });

    app.at("/logout").post(|_| async {
        //user logout
        //check is authenticated
        Ok("created user")
    });
}