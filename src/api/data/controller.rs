use actix_web::{Result, HttpResponse};


pub async fn data() -> Result<HttpResponse> {
  Ok(HttpResponse::Ok().json({}))
}
