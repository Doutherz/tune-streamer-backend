use tide::{Response, Server};

pub fn handle_errors(app: &mut Server<()>) {
    app.with(tide::utils::After(|mut res: Response| async {
        if let Some(error) = res.error() {
            eprintln!("Error occurred: {:?}", error);
            res.set_body(error.to_string())
        }
        Ok(res)
    }));
}