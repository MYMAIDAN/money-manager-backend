pub mod user;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;
pub fn establish_connection() -> PgConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("ERROR: Connection to {}", database_url))
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub fn get_connection_pool(url: &str) -> Pool<ConnectionManager<PgConnection>>{
    let manager = ConnectionManager::<PgConnection>::new(url);
     Pool::builder()
     .test_on_check_out(true)
     .build(manager)
     .expect("Could not build connection pool")

}
