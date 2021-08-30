use chrono::NaiveDateTime;
use diesel::{pg::PgConnection, prelude::*, result::Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AvailUser {
    pub id: i32,
    pub user_name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvailUsers {
    #[serde(rename = "users")]
    pub users: Vec<AvailUser>,
}

impl AvailUsers {
    pub fn show_available_users(connection: &PgConnection) -> Result<Self, Error> {
        use crate::schema::users::dsl::*;
        // Displaying Product details from product table
        let user_list: Vec<AvailUser> = users
            .select((id, user_name, created_at))
            .order_by(created_at.asc())
            .load(connection)
            .expect("Error while loading data from `users` table.");

        Ok(AvailUsers { users: user_list })
    }
}
