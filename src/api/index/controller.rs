use actix_web::{Result, web::Json};
use super::serializers::*;


pub async fn index() -> Result<Json<ResponseData>> {
  Ok(Json(ResponseData {
    data: "Hey, you are in index page!".to_string()
  }))
}
