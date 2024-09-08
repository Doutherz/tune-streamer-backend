use tide::{Response, Server};

pub fn handle_errors(app: &mut Server<()>) {
    app.with(tide::utils::After(|mut res: Response| async {
        if let Some(error) = res.error() {
            eprintln!("Error occurred: {:?}", error);
            res.set_body(error.to_string())
        }
        Ok(res)
    }));

    app.with(tide::utils::After(|mut res: Response| async {

        if res.header("Access-Control-Allow-Origin").is_none() {
            res.append_header("Access-Control-Allow-Origin", "*");
        }
        
        Ok(res)
    }));
}