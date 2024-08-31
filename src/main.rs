use std::env;

mod routes;
mod handlers;
mod services;
mod models;
mod utils;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (ip, port) = if args.len() > 2 {
        (args[1].as_str(), args[2].as_str())
    } else {
        ("0.0.0.0", "8080")
    };

    let mut app = tide::new();

    routes::routes(&mut app);

    utils::error::handle_errors(&mut app);

    println!("Running on: {}:{}", ip, port);
    app.listen(format!("{}:{}", ip, port)).await?;
    

    Ok(())
}
