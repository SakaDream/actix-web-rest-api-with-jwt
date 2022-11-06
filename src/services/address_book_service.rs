use actix_web::web;

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        filters::PersonFilter,
        person::{Person, PersonDTO},
        response::Page,
    },
};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Person>, ServiceError> {
    match Person::find_all(&mut pool.get().unwrap()) {
        Ok(person) => Ok(person),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        }),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Person, ServiceError> {
    match Person::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(person) => Ok(person),
        Err(_) => Err(ServiceError::NotFound {
            error_message: format!("Person with id {} not found", id),
        }),
    }
}

pub fn filter(filter: PersonFilter, pool: &web::Data<Pool>) -> Result<Page<Person>, ServiceError> {
    match Person::filter(filter, &mut pool.get().unwrap()) {
        Ok(people) => Ok(people),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        }),
    }
}

pub fn insert(new_person: PersonDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Person::insert(new_person, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::InternalServerError {
            error_message: constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        }),
    }
}

pub fn update(
    id: i32,
    updated_person: PersonDTO,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    match Person::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match Person::update(id, updated_person, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
            }),
        },
        Err(_) => Err(ServiceError::NotFound {
            error_message: format!("Person with id {} not found", id),
        }),
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Person::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match Person::delete(id, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::InternalServerError {
                error_message: constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            }),
        },
        Err(_) => Err(ServiceError::NotFound {
            error_message: format!("Person with id {} not found", id),
        }),
    }
}
