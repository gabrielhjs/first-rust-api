use actix_web::{Result, web::Json};
use serde::{Serialize};

#[derive(Serialize)]
pub struct Status {
  status: String
}

pub async fn status() -> Result<Json<Status>> {
  Ok(Json(Status {
    status: String::from("Ok")
  }))
}
