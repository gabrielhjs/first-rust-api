use actix_web::{HttpResponse, Result, web};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Serialize;
use sha256::digest;
use uuid::Uuid;

use web_server::models::users::{NewUser, User};
use web_server::schema;
use web_server::schema::users::dsl::id;


use super::{
  serializers::{RequestData, ResponseData, ResponseError, UserId},
  super::super::database::DbPool
};

pub async fn create_user(
  pool: web::Data<DbPool>,
  data: web::Json<RequestData>,
) -> Result<web::HttpResponse> {
  let _conn = pool.get().unwrap();

  let new_user = NewUser {
    id: Uuid::new_v4(),
    username: &data.username,
    password: &*digest(&data.password),
  };

  Ok(match diesel::insert_into(schema::users::table)
    .values(&new_user)
    .get_result::<User>(&_conn) {
    Ok(user) => {
      web::HttpResponse::Created().json(
        ResponseData {
          id: user.id.to_string(),
          username: user.username,
        }
      )
    }
    Err(error) => {
      web::HttpResponse::BadRequest().json(
        ResponseError {
          error: error.to_string()
        }
      )
    }
  })
}


pub async fn delete_user(
  pool: web::Data<DbPool>,
  data: web::Json<UserId>
) -> Result<web::HttpResponse> {
  #[derive(Serialize)]
  struct Response {
    data: String,
  }

  let conn = pool.get().unwrap();

  let count = diesel::delete(
    schema::users::table.filter(
      id.eq(Uuid::parse_str(&data.id).unwrap())
    )
  )
    .execute(&conn)
    .expect("Failed to delete some user");

  Ok(HttpResponse::Accepted().json(
    Response {
      data: format!("Deleted ({}) users", count)
    }
  ))
}
