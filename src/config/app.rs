use crate::api::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configurating routes...");
    cfg.service(
        web::scope("/api")
            .service(ping_controller::ping)
            .service(
                web::scope("/address-book")
                    .service(
                        web::resource("")
                            .route(web::get().to_async(address_book_controller::find_all))
                            .route(web::post().to_async(address_book_controller::insert))
                            .route(web::put().to_async(address_book_controller::update))
                            .route(web::delete().to_async(address_book_controller::delete))
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to_async(address_book_controller::find_by_id))
                    )
                    .service(
                        web::resource("/{query}")
                            .route(web::get().to_async(address_book_controller::query))   
                    )
            )
    );
}