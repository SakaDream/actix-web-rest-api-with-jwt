use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        person::{Person, PersonDTO},
    }
};
use actix_web::{web, http::StatusCode};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Person>, ServiceError> {
    match Person::find_all(&pool.get().unwrap()) {
        Ok(person) => Ok(person),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Person, ServiceError> {
    match Person::find_by_id(id, &pool.get().unwrap()) {
        Ok(person) => Ok(person),
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Person with id {} not found", id))),
    }
}

pub fn query(query: String, pool: &web::Data<Pool>) -> Result<Vec<Person>, ServiceError> {
    match Person::query(query, &pool.get().unwrap()) {
        Ok(people) => Ok(people),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string())),
    }
}

pub fn insert(new_person: PersonDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Person::insert(new_person, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string())),
    }
}

pub fn update(id: i32, updated_person: PersonDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Person::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Person::update(id, updated_person, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string())),
        },
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Person with id {} not found", id))),
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Person::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Person::delete(id, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string())),
        },
        Err(_) => Err(ServiceError::new(StatusCode::NOT_FOUND, format!("Person with id {} not found", id))),
    }
}
