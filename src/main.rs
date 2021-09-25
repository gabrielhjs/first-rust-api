use actix_web::{App, HttpResponse, HttpServer, Result, web, middleware::{Logger, Compress}};
use serde::Serialize;
use dotenv::dotenv;
use std::env;
use actix_web::http::header::ContentEncoding;
use env_logger;

mod database;
mod api;


async fn e404() -> Result<HttpResponse> {
  #[derive(Serialize)]
  struct Error {
    error: String
  }

  Ok(HttpResponse::NotFound()
    .json(Error {
      error: "Page not found".into()
    })
  )
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

  let server = HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .wrap(Compress::new(ContentEncoding::Auto))
      .data(database::establish_connection_pool())
      .default_service(web::get().to(e404))
      .service(web::scope("/").configure(api::routes))
  })
    .workers(2)
    .bind(format!(
      "{}:{}",
      env::var("HOST").unwrap_or("127.0.0.1".to_string()),
      env::var("PORT").unwrap_or("8000".to_string()),
    ))?;

  server.run().await
}
