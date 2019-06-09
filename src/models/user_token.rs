use crate::models::user::LoginInfoDTO;
use jsonwebtoken::Header;

pub static KEY: &'static [u8; 16] = include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

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

impl UserToken {
    pub fn generate_token(login: LoginInfoDTO) -> String {
        let now = time::get_time().sec;
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: login.username,
            login_session: login.login_session,
        };

        jsonwebtoken::encode(&Header::default(), &payload, KEY).unwrap()
    }
}