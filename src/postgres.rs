use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::Connection;
use dotenv::dotenv;
use std::env;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no database url provided");
    // let manager = ConnectionManager::<PgConnection>::new(url);

    // r2d2::Pool::builder()
    //     .build(manager)
    //     .expect("could not build connection pool")

    PgConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to {}", url))
}

// #[derive(Clone, Default)]
pub struct AppState<'a> {
    pub neon_db: &'a PgConnection,
}
