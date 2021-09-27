use actix_web::web;

mod index;
mod status;
mod user;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(index::controller::index)));
    cfg.service(web::resource("/status").route(web::get().to(status::status)));
    cfg.service(
        web::resource("/user")
            .route(web::post().to(user::controller::create_user))
            .route(web::delete().to(user::controller::delete_user)),
    );
}
