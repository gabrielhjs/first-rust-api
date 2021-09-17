use actix_web::{Result, web::{Json, Data}};
use serde::{Serialize};

#[derive(Serialize)]
pub struct User {
  name: String
}

type Pg = r2d2_postgres::r2d2::Pool<r2d2_postgres::PostgresConnectionManager<r2d2_postgres::postgres::NoTls>>;

pub async fn users(pool: Data<Pg>) -> Result<Json<User>> {
  let _conn = pool
    .get()
    .expect("Failed");

  Ok(Json(User {
    name: String::from("an user")
  }))
}