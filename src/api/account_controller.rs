use crate::{
    config::db::Pool,
    constants,
    models::{
        user::{LoginDTO, UserDTO},
        response::ResponseBody,
    },
    services::account_service,
};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::future::{ok, Future};

// POST api/auth/signup
pub fn signup(user_dto: web::Json<UserDTO>, pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    match account_service::signup(user_dto.0, &pool) {
        Ok(message) => ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => ok(err.response()),
    }
}

// POST api/auth/login
pub fn login(login_dto: web::Json<LoginDTO>, pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    match account_service::login(login_dto.0, &pool) {
        Ok(token_res) => ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, token_res))),
        Err(err) => ok(err.response()),
    }
}

// POST api/auth/logout
pub fn logout(req: HttpRequest, pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        account_service::logout(authen_header, &pool);
        ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGOUT_SUCCESS, constants::EMPTY)))
    } else {
        ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
    }
}
