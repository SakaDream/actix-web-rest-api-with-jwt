use crate::{
    config::db::Pool,
    constants,
    models::response::ResponseBody,
    utils::token_utils,
};
use actix_service::{Service, Transform};
use actix_web::{
    Error, HttpResponse,
    dev::{ServiceRequest, ServiceResponse},
};
use futures::{
    Poll,
    future::{ok, Either, FutureResult},
};

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}
pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        // Bypass some account routes
        for ignore_route in constants::IGNORE_ROUTES.into_iter() {
            if req.path().starts_with(ignore_route) {
                return Either::A(self.service.call(req));
            }
        }

        if let Some(pool) = req.app_data::<Pool>() {
            info!("Connecting to database...");
            if let Some(authen_header) = req.headers_mut().get(constants::AUTHORIZATION) {
                info!("Parsing authorization header...");
                if let Ok(authen_str) = authen_header.to_str() {
                    if authen_str.starts_with("bearer") {
                        info!("Parsing token...");
                        let token = authen_str[6..authen_str.len()].trim();
                        if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                            info!("Decoding token...");
                            if token_utils::verify_token(&token_data, &pool).is_ok() {
                                info!("Valid token");
                                return Either::A(self.service.call(req));
                            } else {
                                error!("Invalid token");
                                return Either::B(ok(req.into_response(
                                    HttpResponse::Unauthorized()
                                        .json(ResponseBody::new(constants::MESSAGE_INVALID_TOKEN, constants::EMPTY))
                                        .into_body()
                                )));
                            }
                        }
                    }
                }
            }
        }

        error!("{}", constants::MESSAGE_PROCESS_TOKEN_ERROR);
        Either::B(ok(req.into_response(
            HttpResponse::InternalServerError()
                .json(ResponseBody::new(constants::MESSAGE_PROCESS_TOKEN_ERROR, constants::EMPTY))
                .into_body()
        )))
    }
}
