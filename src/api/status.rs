use actix_web::{HttpResponse, Result};
use serde::{Serialize};

#[derive(Serialize)]
pub struct Status {
  status: String
}

/// Status code of API
pub async fn status() -> Result<HttpResponse> {
  Ok(HttpResponse::Ok().json(Status {
    status: String::from("Ok")
  }))
}
