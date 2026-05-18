use std::env;
use std::sync::OnceLock;

use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::models::user::LoginInfoDTO;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

pub fn get_jwt_secret() -> &'static [u8] {
    static KEY: OnceLock<Vec<u8>> = OnceLock::new();
    KEY.get_or_init(|| {
        let key_str = env::var("JWT_SECRET_KEY").unwrap_or_else(|_| {
            panic!(
                "JWT_SECRET_KEY environment variable must be set.\n\
                 Generate one with: openssl rand -base64 64"
            )
        });
        let known_weak = [
            "", "CHANGE-ME", "secret", "changeme", "jwt_secret",
            "your-secret-key",
        ];
        if known_weak.contains(&key_str.as_str()) {
            panic!(
                "JWT_SECRET_KEY is set to a known default value. \
                 Please generate a secure random key."
            );
        }
        if key_str.len() < 32 {
            panic!(
                "JWT_SECRET_KEY must be at least 32 characters long for security. \
                 Current length: {}",
                key_str.len()
            );
        }
        key_str.into_bytes()
    })
}

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub login_session: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

impl UserToken {
    pub fn generate_token(login: &LoginInfoDTO) -> String {
        dotenv::dotenv().expect("Failed to read .env file");
        let max_age: i64 = match env::var("MAX_AGE") {
            Ok(val) => val.parse::<i64>().unwrap_or(ONE_WEEK),
            Err(_) => ONE_WEEK,
        };

        debug!("Token Max Age: {}", max_age);

        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            exp: now + max_age,
            user: login.username.clone(),
            login_session: login.login_session.clone(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(get_jwt_secret()),
        )
        .unwrap()
    }
}
