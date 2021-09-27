use super::serializers::*;
use actix_web::{web::Json, Result};

pub async fn index() -> Result<Json<ResponseData>> {
    Ok(Json(ResponseData {
        data: "Hey, you are in index page!".to_string(),
    }))
}
