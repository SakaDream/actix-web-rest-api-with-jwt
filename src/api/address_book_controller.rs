use crate::{
    config::db::Pool,
    constants,
    models::response::ResponseBody,
    services::address_book_service,
};
use actix_web::{web, Error, HttpResponse};
use futures::future::{ok, Future};

pub fn find_all(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || address_book_service::find_all(&pool))
        .from_err()
        .and_then(|people| {
            ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people)))
        })
}
