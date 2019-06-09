use crate::{
    config::db::Pool,
    models::{
        user::User,
        user_token::{UserToken, KEY},
    },
};
use actix_web::web;
use jsonwebtoken::{TokenData, Validation};

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(&token, KEY, &Validation::default())
}

pub fn verify_token(token_data: &TokenData<UserToken>, pool: &web::Data<Pool>) -> Result<String, String> {
    if User::is_valid_login_session(&token_data.claims, &pool.get().unwrap()) {
        Ok(token_data.claims.user.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}
