use actix_web::web;

pub mod hello;

mod status;

pub fn routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/status")
      .route(web::get().to(status::status))
  );
}

