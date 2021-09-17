use actix_web::web;

pub mod hello;

mod status;
mod users;

pub fn routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/status")
      .route(web::get().to(status::status))
  );
  cfg.service(
    web::resource("/users")
      .route(web::get().to(users::users))
  );
}

