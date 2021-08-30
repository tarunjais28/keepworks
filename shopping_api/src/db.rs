use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};
use dotenv::dotenv;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_db_pool() -> DbPool {
    dotenv().ok();

    // Read DB credentials from .env file
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    // Setup Connection Manager
    let conection_manager = ConnectionManager::<PgConnection>::new(database_url);
    // Creating DB Pool
    r2d2::Pool::builder()
        .build(conection_manager)
        .expect("Failed to create pool.")
}
