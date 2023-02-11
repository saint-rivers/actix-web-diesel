use actix_web::{get, post, web, HttpResponse, Responder};

use crate::user::model::{Todo, TodoPayload};

#[post("/todos")]
async fn create_todo(payload: web::Json<TodoPayload>) -> impl Responder {
    let todo = web::block(move || {
        // let conn = &mut pool.get()?;
        Todo::add_todo(&payload.title, &payload.content)
    })
    .await
    .unwrap()
    .map_err(actix_web::error::ErrorInternalServerError)
    .unwrap();

    HttpResponse::Ok().json(todo)
}

#[get("/todos")]
async fn get_todos() -> impl Responder {
    let todos = web::block(move || {
        // let conn = &mut pool.get()?;
        Todo::select_todos()
    })
    .await
    .unwrap()
    .map_err(actix_web::error::ErrorInternalServerError)
    .unwrap();

    HttpResponse::Ok().json(todos)
}

pub fn config(config: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(get_todos).service(create_todo);
    config.service(scope);
}
