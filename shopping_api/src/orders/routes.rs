use super::models::*;
use crate::manage_users::*;
use actix_web::{get, post, web, Error, HttpResponse};
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
    result::{DatabaseErrorKind::UniqueViolation, Error::DatabaseError},
};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Register new user with username and password.
#[post("/register")]
async fn register_user(
    pool: web::Data<DbPool>,
    user: web::Json<UserAuth>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Couldn't get db connection from pool");
    let user = match user.into_inner().register_user(&connection) {
        Ok(_) => Ok(()),
        Err(AuthenticationError::DatabaseError(DatabaseError(UniqueViolation, _))) => {
            Err("A user with that name already exists".into())
        }
        Err(e) => Err(convert_auth_error(e)),
    };

    if let Ok(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().body(format!("User not registered: {:?}", user)))
    }
}

// Method for add products from input JSON file and
#[post("/add_products/{user_name}/{password}")]
async fn add_products_in_stock(
    pool: web::Data<DbPool>,
    user_pass: web::Path<UserAuth>,
    products: web::Json<ProductStock>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Couldn't get db connection from pool");

    let user_pass = get_user_and_pass(user_pass);
    // Validate username and password
    match validate_user(user_pass, &connection) {
        Ok(user) => user,
        Err(error) => {
            panic!("Products cannot be added to stock: `{}`.", error);
        }
    };

    let products = products
        .into_inner()
        .insert_products(&connection)
        .expect("Error while parsing JSON file.");

    Ok(HttpResponse::Ok().json(products))
}

// Method for displaying product details
#[get("/products/{user_name}/{password}")]
async fn get_products(
    pool: web::Data<DbPool>,
    user_pass: web::Path<UserAuth>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Couldn't get db connection from pool");

    let user_pass = get_user_and_pass(user_pass);
    // Validate username and password
    match validate_user(user_pass, &connection) {
        Ok(user) => user,
        Err(error) => {
            panic!("Products in stock cannot be displayed: `{}`.", error);
        }
    };

    let products =
        DisplayProducts::get_product_details(&connection).expect("Error while getting products.");

    Ok(HttpResponse::Ok().json(products))
}

// Method for placing order from input JSON file and
#[post("/place_order/{user_name}/{password}")]
async fn place_order(
    pool: web::Data<DbPool>,
    user_pass: web::Path<UserAuth>,
    orders: web::Json<PlaceOrder>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Couldn't get db connection from pool");

    let user_pass = get_user_and_pass(user_pass);
    // Validate username and password
    match validate_user(user_pass, &connection) {
        Ok(user) => user,
        Err(error) => {
            panic!("Order cannot be placed: `{}`.", error);
        }
    };

    let new_order = orders
        .into_inner()
        .place_order(&connection)
        .expect("Error while parsing JSON file.");

    Ok(HttpResponse::Ok().json(new_order))
}

// Method for displaying product details
#[get("/orders/{user_name}/{password}")]
async fn get_orders(
    pool: web::Data<DbPool>,
    user_pass: web::Path<UserAuth>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Couldn't get db connection from pool");

    let user_pass = get_user_and_pass(user_pass);
    // Validate username and password
    match validate_user(user_pass, &connection) {
        Ok(user) => user,
        Err(error) => {
            panic!("Orders cannot be displayed: `{}`.", error);
        }
    };

    let products = Orders::get_orders(&connection).expect("Error while getting orders.");

    Ok(HttpResponse::Ok().json(products))
}

// Method for displaying product details
#[get("/show_users/{user_name}/{password}")]
async fn show_users(
    pool: web::Data<DbPool>,
    user_pass: web::Path<UserAuth>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Couldn't get db connection from pool");

    let user_pass = get_user_and_pass(user_pass);
    // Validate username and password
    match validate_user(user_pass, &connection) {
        Ok(user) => user,
        Err(error) => {
            panic!("Available users cannot be displayed: `{}`.", error);
        }
    };

    let users = AvailUsers::show_available_users(&connection).expect("Error while getting users.");

    Ok(HttpResponse::Ok().json(users))
}
