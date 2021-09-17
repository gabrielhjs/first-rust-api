use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::r2d2::Pool;
use r2d2_postgres::postgres::NoTls;

pub mod schema;
pub mod models;

pub fn establish_connection_pool() -> Pool<PostgresConnectionManager<NoTls>> {
  let manager = PostgresConnectionManager::new(
    "host=localhost user=postgres password=admin123 port=5432".parse().unwrap(),
    r2d2_postgres::postgres::NoTls
  );

  let pool = Pool::new(manager)
    .expect("Failed to create database pool");

  pool.clone()
}