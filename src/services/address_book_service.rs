use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        person::{Person, PersonDTO},
    }
};
use actix_web::{
    http::StatusCode,
    web,
};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Person>, ServiceError> {    
    Person::find_all(&pool.get().unwrap()).map_err(|_| ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA))
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Option<Person>, ServiceError> {
    Person::find_by_id(id, &pool.get().unwrap()).map_err(|_| ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA))
}

pub fn query(query: String, pool: &web::Data<Pool>) -> Result<Vec<Person>, ServiceError> {
    Person::query(query, &pool.get().unwrap()).map_err(|_| ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA))
}

pub fn insert(new_person: PersonDTO, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    Person::insert(new_person, &pool.get().unwrap())
        .map(|_| constants::MESSAGE_OK.to_string())
        .map_err(|_| ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_INSERT_DATA))
}

pub fn update(id: i32, updated_person: PersonDTO, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    match Person::find_by_id(id, &pool.get().unwrap()) {
        Ok(option_person) => {
            if option_person.is_some() {
                Person::update(id, updated_person, &pool.get().unwrap())
                    .map(|_| constants::MESSAGE_OK.to_string())
                    .map_err(|_| ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_UPDATE_DATA))
            } else {
                Err(ServiceError::new(StatusCode::NOT_FOUND, &format!("Person with id {} not found", id)))
            }
        }
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA))
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    match Person::find_by_id(id, &pool.get().unwrap()) {
        Ok(option_person) => {
            if option_person.is_some() {
                Person::delete(id, &pool.get().unwrap())
                    .map(|_| constants::MESSAGE_OK.to_string())
                    .map_err(|_| ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_DELETE_DATA))
            } else {
                Err(ServiceError::new(StatusCode::NOT_FOUND, &format!("Person with id {} not found", id)))
            }
        }
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_CAN_NOT_FETCH_DATA))
    }
}
