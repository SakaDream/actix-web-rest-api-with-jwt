use actix_web::{http::header::HeaderValue, web};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        user::{LoginDTO, User, UserDTO, LoginInfoDTO},
        user_token::UserToken,
    },
    utils::token_utils,
};

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn signup(user: UserDTO, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    match User::signup(user, &mut pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::BadRequest {
            error_message: message,
        }),
    }
}

pub fn login(login: LoginDTO, pool: &web::Data<Pool>) -> Result<TokenBodyResponse, ServiceError> {
    match User::login(login, &mut pool.get().unwrap()) {
        Some(logged_user) => {
            match serde_json::from_value(
                json!({ "token": UserToken::generate_token(&logged_user), "token_type": "bearer" }),
            ) {
                Ok(token_res) => {
                    if logged_user.login_session.is_empty() {
                        Err(ServiceError::Unauthorized {
                            error_message: constants::MESSAGE_LOGIN_FAILED.to_string(),
                        })
                    } else {
                        Ok(token_res)
                    }
                }
                Err(_) => Err(ServiceError::InternalServerError {
                    error_message: constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string(),
                }),
            }
        }
        None => Err(ServiceError::Unauthorized {
            error_message: constants::MESSAGE_USER_NOT_FOUND.to_string(),
        }),
    }
}

pub fn logout(authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    if let Ok(authen_str) = authen_header.to_str() {
        if token_utils::is_auth_header_valid(authen_header) {
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                if let Ok(username) = token_utils::verify_token(&token_data, pool) {
                    if let Ok(user) = User::find_user_by_username(&username, &mut pool.get().unwrap()) {
                        User::logout(user.id, &mut pool.get().unwrap());
                        return Ok(());
                    }
                }
            }
        }
    }

    Err(ServiceError::InternalServerError {
        error_message: constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
    })
}

pub fn me(authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<LoginInfoDTO, ServiceError> {
    if let Ok(authen_str) = authen_header.to_str() {
        if token_utils::is_auth_header_valid(authen_header) {
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                if let Ok(login_info) = User::find_login_info_by_token(&token_data.claims, &mut pool.get().unwrap()) {
                    return Ok(login_info);
                }
            }
        }
    }

    Err(ServiceError::InternalServerError {
        error_message: constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
    })
}
