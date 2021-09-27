use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    pub id: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct ResponseError {
    pub error: String,
}

#[derive(Deserialize)]
pub struct UserId {
    pub id: String,
}
