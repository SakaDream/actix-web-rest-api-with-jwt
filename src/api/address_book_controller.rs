use crate::{
    config::db::Pool,
    constants,
    models::{
        person::PersonDTO,
        response::ResponseBody,
    },
    services::address_book_service,
};
use actix_web::{web, HttpResponse, Result};

// GET api/address-book
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match address_book_service::find_all(&pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/address-book/{id}
pub async fn find_by_id(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match address_book_service::find_by_id(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(person) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, person))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/address-book/query/{query}
pub async fn query(query: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match address_book_service::query(query.into_inner(), &pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/address-book
pub async fn insert(new_person: web::Json<PersonDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match address_book_service::insert(new_person.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// PUT api/address-book/{id}
pub async fn update(id: web::Path<String>, updated_person: web::Json<PersonDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match address_book_service::update(id.into_inner().parse::<i32>().unwrap(), updated_person.0, &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// DELETE api/address-book/{id}
pub async fn delete(id: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match address_book_service::delete(id.into_inner().parse::<i32>().unwrap(), &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}
