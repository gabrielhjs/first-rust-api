use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::r2d2::Pool;
use r2d2_postgres::postgres::NoTls;
use std::env;

pub mod schema;
pub mod models;


#[allow(dead_code)]
pub fn establish_connection_pool() -> Pool<PostgresConnectionManager<NoTls>> {
  let manager = PostgresConnectionManager::new(
    format!(
      "host={} user={} password={} port={}",
      env::var("DB_HOST").expect("Db host is no in environment"),
      env::var("DB_USER").expect("Db user is no in environment"),
      env::var("DB_PASSWORD").expect("Db password is no in environment"),
      env::var("DB_PORT").expect("Db port is no in environment"),
    )
      .parse()
      .unwrap(),
    r2d2_postgres::postgres::NoTls
  );

  let pool = Pool::new(manager)
    .expect("Failed to create database pool");

  pool.clone()
}
