use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseData {
    pub data: String,
}
