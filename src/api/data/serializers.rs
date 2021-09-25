use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestData {
  pub data: String
}

#[derive(Serialize)]
pub struct ResponseData {
  pub data: String
}
