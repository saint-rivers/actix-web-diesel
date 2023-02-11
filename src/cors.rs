use actix_cors::Cors;
use actix_web::http::header;

pub fn cors_config() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_origin("http://localhost:3000/")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .supports_credentials()
}
