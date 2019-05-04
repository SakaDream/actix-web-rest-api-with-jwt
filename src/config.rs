use crate::api::ping_controller;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(ping_controller::ping)
    );
}
