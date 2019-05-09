use crate::{
    config::db::Pool,
    constants,
    models::{
        person::PersonDTO,
        response::ResponseBody,
    },
    services::address_book_service,
};
use actix_web::{web, Error, HttpResponse};
use futures::future::{ok, Future};

// GET api/address-book
pub fn find_all(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    match address_book_service::find_all(&pool) {
        Ok(people) => ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => ok(err.response()),
    }
}

// GET api/address-book/{id}
pub fn find_by_id(id: web::Path<String>, pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    match address_book_service::find_by_id(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(person) => ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, person))),
        Err(err) => ok(err.response()),
    }
}

// GET api/address-book/{query}
pub fn query(query: web::Path<String>, pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    match address_book_service::query(query.into_inner(), &pool) {
        Ok(people) => ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => ok(err.response()),
    }
}

// POST api/address-book
pub fn insert(new_person: web::Json<PersonDTO>, pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    match address_book_service::insert(new_person.0, &pool) {
        Ok(message) => ok(HttpResponse::Created().json(ResponseBody::new(constants::MESSAGE_OK, message))),
        Err(err) => ok(err.response()),
    }
}

// PUT api/address-book/{id}
pub fn update(id: web::Path<String>, updated_person: web::Json<PersonDTO>, pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    match address_book_service::update(id.into_inner().parse::<i32>().unwrap(), updated_person.0, &pool) {
        Ok(message) => ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, message))),
        Err(err) => ok(err.response()),
    }
}

// DELETE api/address-book/{id}
pub fn delete(id: web::Path<String>, pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    match address_book_service::delete(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(message) => ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, message))),
        Err(err) => ok(err.response()),
    }
}
