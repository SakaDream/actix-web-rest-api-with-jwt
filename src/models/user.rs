use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::{prelude::*, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::db::Connection,
    constants,
    models::{login_history::LoginHistory, user_token::UserToken},
    schema::users::{self, dsl::*},
};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub login_session: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct LoginInfoDTO {
    pub username: String,
    pub login_session: String,
}

impl User {
    pub fn signup(new_user: UserDTO, conn: &mut Connection) -> Result<String, String> {
        if Self::find_user_by_username(&new_user.username, conn).is_err() {
            let new_user = UserDTO {
                password: hash(&new_user.password, DEFAULT_COST).unwrap(),
                ..new_user
            };
            diesel::insert_into(users).values(new_user).execute(conn);
            Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
        } else {
            Err(format!(
                "User '{}' is already registered",
                &new_user.username
            ))
        }
    }

    pub fn login(login: LoginDTO, conn: &mut Connection) -> Option<LoginInfoDTO> {
        if let Ok(user_to_verify) = users
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(conn)
        {
            if !user_to_verify.password.is_empty()
                && verify(&login.password, &user_to_verify.password).unwrap()
            {
                if let Some(login_history) = LoginHistory::create(&user_to_verify.username, conn) {
                    if LoginHistory::save_login_history(login_history, conn).is_err() {
                        return None;
                    }
                    let login_session_str = User::generate_login_session();
                    if User::update_login_session_to_db(
                        &user_to_verify.username,
                        &login_session_str,
                        conn,
                    ) {
                        return Some(LoginInfoDTO {
                            username: user_to_verify.username,
                            login_session: login_session_str,
                        });
                    }
                }
            } else {
                return Some(LoginInfoDTO {
                    username: user_to_verify.username,
                    login_session: String::new(),
                });
            }
        }

        None
    }

    pub fn logout(user_id: i32, conn: &mut Connection) {
        if let Ok(user) = users.find(user_id).get_result::<User>(conn) {
            Self::update_login_session_to_db(&user.username, "", conn);
        }
    }

    pub fn is_valid_login_session(user_token: &UserToken, conn: &mut Connection) -> bool {
        users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn find_login_info_by_token(user_token: &UserToken, conn: &mut Connection) -> Result<LoginInfoDTO, String> {
        let user_result = users
        .filter(username.eq(&user_token.user))
        .filter(login_session.eq(&user_token.login_session))
        .get_result::<User>(conn);

        if let Ok(user) = user_result {
            return Ok(LoginInfoDTO {
                username: user.username,
                login_session: user.login_session,
            });
        }

        Err("User not found!".to_string())
    }

    pub fn find_user_by_username(un: &str, conn: &mut Connection) -> QueryResult<User> {
        users.filter(username.eq(un)).get_result::<User>(conn)
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn update_login_session_to_db(
        un: &str,
        login_session_str: &str,
        conn: &mut Connection,
    ) -> bool {
        if let Ok(user) = User::find_user_by_username(un, conn) {
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
    }
}
