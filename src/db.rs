use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type PostgresPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn establish_connection() -> PostgresPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("no database url provided");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("could not build connection pool")
}
