// #[macro_use]
extern crate diesel;

mod db;
mod schema;
mod user;
mod cors;

use std::env;

use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }
    // creates a CLI logger
    env_logger::init();

    // check if env variables are present
    dotenv().ok();
    let host = env::var("HOST").expect("host is not set");
    let port = env::var("PORT").expect("port is not set");

    // configure the database connection with our AppState
    // let todo_db = AppState::init();
    // let app_data = web::Data::new(todo_db);

    // let pool = establish_connection();

    println!("Server started successfully");

    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(pool.clone()))
            .configure(user::routes::config)
            .wrap(cors::cors_config())
            .wrap(Logger::default())
    })
    .bind((host, port.parse().expect("not a valid port number")))?
    .run()
    .await
}
