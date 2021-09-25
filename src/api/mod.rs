use actix_web::web;

mod status;
mod users;
mod index;
mod hello;
mod stream;
mod data;


pub fn routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("")
      .route(web::get().to(index::controller::index))
  );
  cfg.service(
    web::resource("/status")
      .route(web::get().to(status::status)),
  );
  cfg.service(
    web::resource("/users")
      .route(web::get().to(users::users))
  );
  cfg.service(
    web::resource("/hello")
      .route(web::post().to(hello::controller::hello))
  );
  cfg.service(
    web::resource("/stream")
      .route(web::get().to(stream::controller::stream))
  );
  cfg.service(
    web::resource("/data")
      .route(web::get().to(data::controller::data))
  );
}

