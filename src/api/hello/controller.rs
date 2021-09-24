use actix_web::{Result, web::Json};
use super::serializers::*;


pub async fn hello(data: Json<RequestData>) -> Result<Json<ResponseData>> {
  Ok(Json(ResponseData {
    data: format!("Hey, you said: {}", data.data)
  }))
}
