pub mod auth;
pub mod show_users;

pub use auth::*;
pub use show_users::*;

use actix_web::web;
use diesel::pg::PgConnection;
use std::error::Error;

pub fn validate_user(user: UserAuth, connection: &PgConnection) -> Result<User, Box<dyn Error>> {
    match user.verify_user(connection) {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err("No user found with the given username".into()),
        Err(e) => Err(convert_auth_error(e)),
    }
}

pub fn convert_auth_error(error: AuthenticationError) -> Box<dyn Error> {
    use AuthenticationError::*;

    match error {
        IncorrectPassword => "The password given does not match our records".into(),
        BcryptError(err) => err.into(),
        DatabaseError(err) => err.into(),
    }
}

pub fn get_user_and_pass(user_pass: web::Path<UserAuth>) -> UserAuth {
    UserAuth {
        user_name: user_pass.user_name.to_string(),
        password: user_pass.password.to_string(),
    }
}
