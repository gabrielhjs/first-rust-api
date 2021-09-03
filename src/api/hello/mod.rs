use actix_web::web;

mod controllers;

pub fn routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("")
      .route(web::post().to(controllers::hello))
  );
}
