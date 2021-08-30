use crate::schema::*;
use bcrypt::*;
use diesel::{self, insert_into, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AuthenticationError {
    IncorrectPassword,
    BcryptError(BcryptError),
    DatabaseError(diesel::result::Error),
}

impl From<BcryptError> for AuthenticationError {
    fn from(e: BcryptError) -> Self {
        AuthenticationError::BcryptError(e)
    }
}

pub use self::AuthenticationError::IncorrectPassword;

#[derive(Queryable, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub user_name: String,
}

#[derive(Queryable)]
pub struct UserWithPassword {
    user: User,
    password: String,
}

#[derive(Queryable, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserAuth {
    pub user_name: String,
    pub password: String,
}

impl UserAuth {
    pub fn verify_user(
        self,
        connection: &PgConnection,
    ) -> Result<Option<User>, AuthenticationError> {
        let user_and_password = users::table
            .filter(users::user_name.eq(self.user_name))
            .select(((users::id, users::user_name), users::hashed_password))
            .first::<UserWithPassword>(connection)
            .optional()
            .map_err(AuthenticationError::DatabaseError)?;

        if let Some(user_and_password) = user_and_password {
            if verify(self.password, &user_and_password.password)? {
                Ok(Some(user_and_password.user))
            } else {
                Err(IncorrectPassword)
            }
        } else {
            Ok(None)
        }
    }

    pub fn register_user(self, connection: &PgConnection) -> Result<User, AuthenticationError> {
        let hash_pass = hash(self.password, DEFAULT_COST)?;
        use crate::schema::users::dsl::*;
        insert_into(users)
            .values((user_name.eq(self.user_name), hashed_password.eq(hash_pass)))
            .returning((id, user_name))
            .get_result(connection)
            .map_err(AuthenticationError::DatabaseError)
    }
}
