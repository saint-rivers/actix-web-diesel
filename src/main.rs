// #[macro_use]
extern crate diesel;

mod handler;
mod model;
mod postgres;
mod schema;

use std::env;

use actix_cors::Cors;
use actix_web::http::header;
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

    println!("Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3000/")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            // .app_data(web::Data::new(pool))
            // .app_data(web::Data::new(pool.to_owned()))
            .configure(handler::init_routes)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind((host, port.parse().expect("not a valid port number")))?
    .run()
    .await
}
