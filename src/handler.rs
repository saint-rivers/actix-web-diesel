use actix_web::{post, web, Error, HttpResponse};
use diesel::prelude::*;

use crate::{
    model::{NewTodo, Todo, TodoPayload},
    postgres::PostgresPool,
};

type DbError = Box<dyn std::error::Error + Send + Sync>;

fn add_todo(_title: &str, _content: &str, conn: &mut PgConnection) -> Result<Todo, DbError> {
    use crate::schema::todos::dsl::*;

    let date_time = chrono::Local::now().naive_local();
    let new_todo = NewTodo {
        title: _title,
        content: _content,
        completed: false,
        created_at: date_time,
        updated_at: date_time,
    };

    // let conn = &mut establish_connection();

    let res = diesel::insert_into(todos)
        .values(&new_todo)
        .get_result(conn)
        .expect("Error saving new post");

    Ok(res)
}

#[post("/todos")]
async fn create_todo(
    pool: web::Data<PostgresPool>,
    payload: web::Json<TodoPayload>,
) -> Result<HttpResponse, Error> {
    // let todo = web::block(move || {
    //     let conn = conn.to_owned();
    //     add_todo(&payload.title, &payload.content, &conn)
    // })

    let todo = web::block(move || {
        let conn = &mut pool.get()?;
        add_todo(&payload.title, &payload.content, conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(todo))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(create_todo);
    config.service(scope);
}
