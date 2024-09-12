use tide::{Response, Server};

pub fn handle_logging(app: &mut Server<()>) {
    
    

    app.with(tide::utils::After(|mut res: Response| async {

        if res.header("Access-Control-Allow-Origin").is_none() {
            res.append_header("Access-Control-Allow-Origin", "*");
        }
        
        Ok(res)
    }));
}