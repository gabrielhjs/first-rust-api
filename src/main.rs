use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use dotenv::dotenv;
use std::env;

mod api;

async fn e404() -> impl Responder {
  HttpResponse::Ok()
    .body("Page not found, please go to /")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();


  let server = HttpServer::new(|| {
    App::new()
      .default_service(web::get().to(e404))
      .service(web::scope("/hello").configure(api::hello::routes))
      .service(web::scope("/api").configure(api::routes))
  })
    .bind(format!(
      "{}:{}",
      env::var("HOST").unwrap_or("127.0.0.1".to_string()),
      env::var("PORT").unwrap_or("8080".to_string()),
    ))?;

  println!(
    "Server running on {}:{}",
    env::var("HOST").unwrap_or("127.0.0.1".to_string()),
    env::var("PORT").unwrap_or("8080".to_string()),
  );

  server.run().await
}
