use diesel::{prelude::*};

use crate::{db::{DbError, self}, schema::todos};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Todo {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// domain
impl Todo {
    pub fn add_todo(_title: &str, _content: &str) -> Result<Todo, DbError> {
        use crate::schema::todos::dsl::*;

        let date_time = chrono::Local::now().naive_local();
        let new_todo = NewTodo {
            title: _title,
            content: _content,
            completed: false,
            created_at: date_time,
            updated_at: date_time,
        };

        let conn = &mut db::establish_connection().get()?;

        let res = diesel::insert_into(todos)
            .values(&new_todo)
            .get_result(conn)
            .expect("Error saving new post");

        Ok(res)
    }

    pub fn select_todos() -> Result<Vec<Todo>, DbError> {
        use crate::schema::todos::dsl::*;
        let conn = &mut db::establish_connection().get()?;

        let res = todos
            .limit(5)
            .load::<Todo>(conn)
            .expect("error loading todos");

        Ok(res)
    }
}

// this gets inserted into the database
#[derive(Debug, Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// this is the request
#[derive(Debug, Deserialize, Serialize)]
pub struct TodoPayload {
    // pub id: String,
    pub title: String,
    pub content: String,
}
