use std::env;

use utils::load_music::load_music;

mod routes;
mod handlers;
mod services;
mod models;
mod utils;
mod middleware;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (ip, port) = if args.len() > 2 {
        (args[1].as_str(), args[2].as_str())
    } else {
        ("0.0.0.0", "8080")
    };

    if args.len() == 2 {
        if args[1].as_str() == "reload" {
            match load_music().await {
                Ok(_) => println!("Music loaded successfully"),
                Err(e) => println!("Music loader error: {}", e)
            }
        }   
    }

    let mut app = tide::new();

    routes::routes(&mut app);

    utils::error::handle_errors(&mut app);

    println!("Running on: {}:{}", ip, port);
    app.listen(format!("{}:{}", ip, port)).await?;
    

    Ok(())
}
