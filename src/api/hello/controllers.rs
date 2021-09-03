use actix_web::{Result, web::Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestData {
  data: String
}

#[derive(Serialize)]
pub struct ResponseData {
  data: String
}

pub async fn hello(data: Json<RequestData>) -> Result<Json<ResponseData>> {
  Ok(Json(ResponseData {
    data: format!("Hey, you said: {}", data.data)
  }))
}
