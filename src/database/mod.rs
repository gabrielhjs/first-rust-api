use std::env;

use diesel::r2d2::ConnectionManager;
use r2d2_postgres::r2d2::Pool;

pub mod repositories;

pub type DbPool = Pool<ConnectionManager<diesel::PgConnection>>;


#[allow(dead_code)]
pub fn establish_connection_pool() -> DbPool {
  let manager = ConnectionManager::<diesel::PgConnection>::new(
    env::var("DATABASE_URL").expect("DATABASE_URL env must to be set")
  );

  let pool = Pool::new(manager)
    .expect("Failed to create database pool");

  pool.clone()
}
